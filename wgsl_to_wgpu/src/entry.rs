use naga::{Function, Module};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::wgsl::vertex_entry_structs;

pub fn fragment_target_count(module: &Module, f: &Function) -> usize {
    match &f.result {
        Some(r) => match &r.binding {
            Some(b) => {
                // Builtins don't have render targets.
                if matches!(b, naga::Binding::Location { .. }) {
                    1
                } else {
                    0
                }
            }
            None => {
                // Fragment functions should return a single variable or a struct.
                match &module.types[r.ty].inner {
                    naga::TypeInner::Struct { members, .. } => members
                        .iter()
                        .filter(|m| matches!(m.binding, Some(naga::Binding::Location { .. })))
                        .count(),
                    _ => 0,
                }
            }
        },
        None => 0,
    }
}

pub fn vertex_inputs(
    entry_point: &naga::EntryPoint,
    module: &naga::Module,
) -> Vec<(String, Ident)> {
    let vertex_inputs = vertex_entry_structs(entry_point, module);
    vertex_inputs
        .into_iter()
        .map(|input| (input.name, input.type_name))
        .collect()
}

pub fn entry_point_constants(module: &naga::Module) -> TokenStream {
    let fragment_state = {
        let (variants, entries): (Vec<TokenStream>, Vec<TokenStream>) = module
            .entry_points
            .iter()
            .filter(|e| e.stage == naga::ShaderStage::Fragment)
            .map(|e| {
                let name = e.name.as_str();
                let variant_name = Ident::new(name, Span::call_site());
                let num_targets = fragment_target_count(module, &e.function);
                (
                    quote! {#variant_name {
                        targets: [Option<wgpu::ColorTargetState>; #num_targets],
                    }},
                    quote! {
                        Self::#variant_name { targets } => (#name, targets)
                    },
                )
            })
            .unzip();

        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            pub enum FragmentEntry {
                #(#variants),*
            }
            impl FragmentEntry {
                pub fn entry_point_and_targets(&self) -> (&'static str, &[Option<wgpu::ColorTargetState>]) {
                    match self {
                        #(#entries,)*
                        _ => unreachable!(),
                    }
                }
            }
        }
    };

    quote! {
        // #vertex_state
        #fragment_state
    }
}

pub fn vertex_struct_methods(module: &naga::Module) -> TokenStream {
    let structs = vertex_input_structs(module);
    quote!(#(#structs)*)
}

fn vertex_input_structs(module: &naga::Module) -> Vec<TokenStream> {
    let vertex_inputs = crate::wgsl::get_vertex_input_structs(module);
    vertex_inputs.iter().map(|input|  {
        let name = &input.type_name;

        let count = Literal::usize_unsuffixed(input.fields.len());
        let attributes: Vec<_> = input
            .fields
            .iter()
            .map(|(location, m)| {
                let field_name: TokenStream = m.name.as_ref().unwrap().parse().unwrap();
                let location = Literal::usize_unsuffixed(*location as usize);
                let format = crate::wgsl::vertex_format(&module.types[m.ty]);
                // TODO: Will the debug implementation always work with the macro?
                let format = Ident::new(&format!("{format:?}"), Span::call_site());

                quote! {
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::#format,
                        offset: std::mem::offset_of!(#name, #field_name) as u64,
                        shader_location: #location,
                    }
                }
            })
            .collect();


        // The vertex_attr_array! macro doesn't account for field alignment.
        // Structs with glam::Vec4 and glam::Vec3 fields will not be tightly packed.
        // Manually calculate the Rust field offsets to support using bytemuck for vertices.
        // This works since we explicitly mark all generated structs as repr(C).
        // Assume elements are in Rust arrays or slices, so use size_of for stride.
        // TODO: Should this enforce WebGPU alignment requirements for compatibility?
        // https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-gpuvertexbufferlayout

        // TODO: Support vertex inputs that aren't in a struct.
        quote! {
            impl #name {
                pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; #count] = [#(#attributes),*];

                pub const fn vertex_buffer_layout(step_mode: wgpu::VertexStepMode) -> wgpu::VertexBufferLayout<'static> {
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<#name>() as u64,
                        step_mode,
                        attributes: &#name::VERTEX_ATTRIBUTES
                    }
                }
            }
        }
    }).collect()
}

#[cfg(test)]
mod test {
    

    
    
}
