use naga::Override;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    wgsl::{require_ordered_float, rust_type},
    MatrixVectorTypes,
};

pub fn consts(module: &naga::Module) -> Vec<TokenStream> {
    // Create matching Rust constants for WGSl constants.
    module
        .constants
        .iter()
        .filter_map(|(_, t)| -> Option<TokenStream> {
            let name = Ident::new(t.name.as_ref()?, Span::call_site());

            // TODO: Add support for f64 and f16 once naga supports them.
            let type_and_value = match &module.global_expressions[t.init] {
                naga::Expression::Literal(literal) => match literal {
                    naga::Literal::F64(v) => Some(quote!(f32 = #v)),
                    naga::Literal::F32(v) => Some(quote!(f32 = #v)),
                    naga::Literal::U32(v) => Some(quote!(u32 = #v)),
                    naga::Literal::I32(v) => Some(quote!(i32 = #v)),
                    naga::Literal::U64(v) => Some(quote!(u64 = #v)),
                    naga::Literal::Bool(v) => Some(quote!(bool = #v)),
                    naga::Literal::I64(v) => Some(quote!(i64 = #v)),
                    naga::Literal::AbstractInt(v) => Some(quote!(i64 = #v)),
                    naga::Literal::AbstractFloat(v) => Some(quote!(f64 = #v)),
                },
                _ => None,
            }?;

            Some(quote!( pub const #name: #type_and_value;))
        })
        .collect()
}

fn convert_overridable_constant_to_f64(ty: &naga::Type, value: TokenStream) -> TokenStream {
    match ty.inner {
        naga::TypeInner::Scalar(s) if s.kind == naga::ScalarKind::Bool => {
            quote!(if #value { 1f64 } else { 0f64 })
        }
        naga::TypeInner::Scalar(s) if require_ordered_float(s.kind) => {
            quote!(#value.into_inner() as f64)
        }
        _ => quote!(#value as f64),
    }
}

fn convert_overridable_constant_to_pair(module: &naga::Module, o: &Override, value: TokenStream) -> TokenStream {
    let key = override_key(o);
    let ty = &module.types[o.ty];
    let value = convert_overridable_constant_to_f64(ty, value);
    quote!((#key.to_owned(), #value))
}

pub fn pipeline_overridable_constants(module: &naga::Module) -> TokenStream {
    let overrides: Vec<_> = module.overrides.iter().map(|(_, o)| o).collect();

    let fields: Vec<_> = overrides
        .iter()
        .map(|o| {
            let name = Ident::new(o.name.as_ref().unwrap(), Span::call_site());
            // TODO: Do we only need to handle scalar types here?
            let ty = rust_type(
                module,
                &module.types[o.ty],
                MatrixVectorTypes::Rust { ordered: true },
            );

            if o.init.is_some() {
                quote!(pub #name: Option<#ty>)
            } else {
                quote!(pub #name: #ty)
            }
        })
        .collect();

    let entries: Vec<_> = overrides
        .iter()
        .map(|o| {
            let name = Ident::new(o.name.as_ref().unwrap(), Span::call_site());
            if o.init.is_some() {
                let pair = convert_overridable_constant_to_pair(module, o, quote!(v));
                quote!(self.#name.map(|v| #pair))
            } else {
                let pair = convert_overridable_constant_to_pair(module, o, quote!(self.#name));
                quote!(Some(#pair))
            }
        })
        .collect();

    // Create a Rust struct that can initialize the constants dictionary.
    quote! {
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OverrideConstants {
            #(#fields),*
        }

        impl OverrideConstants {
            pub fn constants(&self) -> std::collections::HashMap<String, f64> {
                [#(#entries),*].into_iter().filter_map(|a| a).collect()
            }
        }
    }
}

fn override_key(o: &naga::Override) -> String {
    // The @id(id) should be the name if present.
    o.id.map(|i| i.to_string())
        .unwrap_or(o.name.clone().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_tokens_eq;
    use indoc::indoc;

    #[test]
    fn write_global_constants() {
        let source = indoc! {r#"
            const INT_CONST = 12;
            const UNSIGNED_CONST = 34u;
            const FLOAT_CONST = 0.1;
            // TODO: Naga doesn't implement f16, even though it's in the WGSL spec
            // const SMALL_FLOAT_CONST:f16 = 0.1h;
            const BOOL_CONST = true;

            @fragment
            fn main() {
                // TODO: This is valid WGSL syntax, but naga doesn't support it apparently.
                // const C_INNER = 456;
            }
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();

        let consts = consts(&module);
        let actual = quote!(#(#consts)*);

        assert_tokens_eq!(
            quote! {
                pub const INT_CONST: i32 = 12i32;
                pub const UNSIGNED_CONST: u32 = 34u32;
                pub const FLOAT_CONST: f32 = 0.1f32;
                pub const BOOL_CONST: bool = true;
            },
            actual
        );
    }

    #[test]
    fn write_pipeline_overrideable_constants() {
        let source = indoc! {r#"
            override b1: bool = true;
            override b2: bool = false;
            override b3: bool;

            override f1: f32 = 0.5;
            override f2: f32;

            // override f3: f64 = 0.6;
            // override f4: f64;

            override i1: i32 = 0;
            override i2: i32;
            override i3: i32 = i1 * i2;

            @id(0) override a: f32 = 1.0;
            @id(35) override b: f32 = 2.0;

            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();

        let actual = pipeline_overridable_constants(&module);

        assert_tokens_eq!(
            quote! {
                #[derive(Default)]
                pub struct OverrideConstants {
                    pub b1: Option<bool>,
                    pub b2: Option<bool>,
                    pub b3: bool,
                    pub f1: Option<f32>,
                    pub f2: f32,
                    pub i1: Option<i32>,
                    pub i2: i32,
                    pub i3: Option<i32>,
                    pub a: Option<f32>,
                    pub b: Option<f32>,
                }

                impl OverrideConstants {
                    pub fn constants(&self) -> std::collections::HashMap<String, f64> {
                        let mut entries = std::collections::HashMap::from([
                            ("b3".to_owned(), if self.b3 { 1.0 } else { 0.0 }),
                            ("f2".to_owned(), self.f2 as f64),
                            ("i2".to_owned(), self.i2 as f64)
                        ]);
                        if let Some(value) = self.b1 {
                            entries.insert("b1".to_owned(), if value { 1.0 } else { 0.0 });
                        };
                        if let Some(value) = self.b2 {
                            entries.insert("b2".to_owned(), if value { 1.0 } else { 0.0 });
                        };
                        if let Some(value) = self.f1 {
                            entries.insert("f1".to_owned(), value as f64);
                        };
                        if let Some(value) = self.i1 {
                            entries.insert("i1".to_owned(), value as f64);
                        };
                        if let Some(value) = self.i3 {
                            entries.insert("i3".to_owned(), value as f64);
                        };
                        if let Some(value) = self.a {
                            entries.insert("0".to_owned(), value as f64);
                        };
                        if let Some(value) = self.b {
                            entries.insert("35".to_owned(), value as f64);
                        }
                        entries
                    }
                }
            },
            actual
        );
    }

    #[test]
    fn write_pipeline_overrideable_constants_empty() {
        let source = indoc! {r#"
            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();
        let actual = pipeline_overridable_constants(&module);
        assert_tokens_eq!(quote!(), actual);
    }
}
