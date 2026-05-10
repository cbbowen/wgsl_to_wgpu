use naga::Override;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    MatrixVectorTypes, WriteOptions,
    wgsl::{require_ordered_float, rust_type},
};

pub fn consts(module: &naga::Module, options: &WriteOptions) -> Vec<TokenStream> {
    // Create matching Rust constants for WGSl constants.
    module
        .constants
        .iter()
        .filter_map(|(_, t)| -> Option<TokenStream> {
            let name = Ident::new(options.undecorate(t.name.as_ref()?), Span::call_site());

            let type_and_value = match &module.global_expressions[t.init] {
                naga::Expression::Literal(literal) => match literal {
                    naga::Literal::F64(v) => Some(quote!(f64 = #v)),
                    naga::Literal::F32(v) => Some(quote!(f32 = #v)),
                    naga::Literal::F16(v) => {
                        let v = v.to_f64();
                        Some(quote!(::half::f16 = ::half::f16::from_f64_const(#v)))
                    }
                    // wgpu 29.0
                    // naga::Literal::U16(v) => Some(quote!(u16 = #v)),
                    // naga::Literal::I16(v) => Some(quote!(i16 = #v)),
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

fn convert_overridable_constant_to_pair(
    module: &naga::Module,
    o: &Override,
    value: TokenStream,
) -> TokenStream {
    let key = override_key(o);
    let ty = &module.types[o.ty];
    let value = convert_overridable_constant_to_f64(ty, value);
    quote!((#key, #value))
}

pub fn pipeline_overridable_constants(
    module: &naga::Module,
    options: &WriteOptions,
) -> TokenStream {
    let overrides: Vec<_> = module.overrides.iter().map(|(_, o)| o).collect();

    let fields: Vec<_> = overrides
        .iter()
        .map(|o| {
            let name = o.name.as_ref().unwrap();
            let name = Ident::new(options.undecorate(name), Span::call_site());
            // TODO: Do we only need to handle scalar types here?
            let ty = rust_type(
                module,
                &module.types[o.ty],
                MatrixVectorTypes::Rust { ordered: true },
                options,
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
            let name = o.name.as_ref().unwrap();
            let name = Ident::new(options.undecorate(name), Span::call_site());
            if o.init.is_some() {
                let pair = convert_overridable_constant_to_pair(module, o, quote!(v));
                quote!(self.#name.map(|v| #pair))
            } else {
                let pair = convert_overridable_constant_to_pair(module, o, quote!(self.#name));
                quote!(Some(#pair))
            }
        })
        .collect();

    let constants = if entries.is_empty() {
        quote!(vec![])
    } else {
        quote!([#(#entries),*].into_iter().flatten().collect())
    };

    // Create a Rust struct that can initialize the constants dictionary.
    quote! {
        #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OverrideConstants {
            #(#fields),*
        }

        impl OverrideConstants {
            pub fn constants(&self) -> Vec<(&'static str, f64)> {
                #constants
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
            enable f16;

            const INT_CONST = 12;
            const UNSIGNED_CONST = 34u;
            const FLOAT_CONST = 0.5;
            const SMALL_FLOAT_CONST: f16 = 0.5h;
            const BOOL_CONST = true;

            @fragment
            fn main() {
                // TODO: This is valid WGSL syntax, but naga doesn't support it apparently.
                // const C_INNER = 456;
            }
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();

        let consts = consts(&module, &WriteOptions::default());
        let actual = quote!(#(#consts)*);

        assert_tokens_eq!(
            quote! {
                // pub const INT_CONST: i32 = 12i32;
                pub const UNSIGNED_CONST: u32 = 34u32;
                // pub const FLOAT_CONST: f32 = 0.5f32;
                pub const SMALL_FLOAT_CONST: ::half::f16 = ::half::f16::from_f64_const(0.5f64);
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

        let actual = pipeline_overridable_constants(&module, &WriteOptions::default());

        assert_tokens_eq!(
            quote! {
                #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct OverrideConstants {
                    pub b1: Option<bool>,
                    pub b2: Option<bool>,
                    pub b3: bool,
                    pub f1: Option<ordered_float::OrderedFloat<f32>>,
                    pub f2: ordered_float::OrderedFloat<f32>,
                    pub i1: Option<i32>,
                    pub i2: i32,
                    pub i3: Option<i32>,
                    pub a: Option<ordered_float::OrderedFloat<f32>>,
                    pub b: Option<ordered_float::OrderedFloat<f32>>,
                }

                impl OverrideConstants {
                    pub fn constants(&self) -> Vec<(&'static str, f64)> {
                        [
                            self.b1.map(|v| ("b1", if v { 1f64 } else { 0f64 })),
                            self.b2.map(|v| ("b2", if v { 1f64 } else { 0f64 })),
                            Some(("b3", if self.b3 { 1f64 } else { 0f64 })),
                            self.f1.map(|v| ("f1", v.into_inner() as f64)),
                            Some(("f2", self.f2.into_inner() as f64)),
                            self.i1.map(|v| ("i1", v as f64)),
                            Some(("i2", self.i2 as f64)),
                            self.i3.map(|v| ("i3", v as f64)),
                            self.a.map(|v| ("0", v.into_inner() as f64)),
                            self.b.map(|v| ("35", v.into_inner() as f64)),
                        ]
                        .into_iter()
                        .flatten()
                        .collect()
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
        let actual = pipeline_overridable_constants(&module, &WriteOptions::default());
        assert_tokens_eq!(
            quote! {
                #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct OverrideConstants {}
                impl OverrideConstants {
                    pub fn constants(&self) -> Vec<(&'static str, f64)> {
                        vec![]
                    }
                }
            },
            actual
        );
    }
}
