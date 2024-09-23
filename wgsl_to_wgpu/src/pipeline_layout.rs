use crate::wgsl::VertexInput;

use super::bindgroup::BindGroup;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::Ident;

struct PipelineData {
    pipeline_cache: Ident,
    pipeline_key: Ident,
    pipeline_key_definition: TokenStream,
    pipeline_impl_definitions: TokenStream,
}

fn define_render_pipeline_key(entry_name: &str, step_args: &[Ident]) -> (TokenStream, Ident) {
    let name = Ident::new(&format!("PipelineKey_{}", entry_name), Span::call_site());
    (
        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            struct #name {
                #(#step_args: wgpu::VertexStepMode,)*
                overrides: OverrideConstants,
                primitive: wgpu::PrimitiveState,
                depth_stencil: Option<wgpu::DepthStencilState>,
                multisample: wgpu::MultisampleState,
                fragment: FragmentEntry,
                multiview: Option<std::num::NonZero<u32>>,
            }
        },
        name,
    )
}

fn define_create_render_pipeline(
    module: &naga::Module,
    entry: &naga::EntryPoint,
) -> PipelineData {
    let structs = super::wgsl::vertex_entry_structs(entry, module);
    let entry_name = &entry.name;

    let pipeline_cache = Ident::new(&format!("{}_pipelines", entry_name), Span::call_site());
    let function_name = Ident::new(&format!("{}_pipeline", entry_name), Span::call_site());
    let from_key_name = Ident::new(
        &format!("pipeline_{}_from_key", entry_name),
        Span::call_site(),
    );

    let step_args: Vec<_> = structs
        .iter()
        .map(|input| Ident::new(&format!("{}_step_mode", input.name), Span::call_site()))
        .collect();
    let (pipeline_key_definition, pipeline_key) =
        define_render_pipeline_key(entry_name, &step_args);

    let vertex_buffer_layouts =
        structs
            .iter()
            .zip(step_args.iter())
            .map(|(VertexInput { type_name, .. }, step_arg)| {
                quote!(#type_name::vertex_buffer_layout(#step_arg))
            });
    let pipeline_impl_definitions = quote! {
        fn #from_key_name(
                &self,
                #pipeline_key {
                    #(#step_args,)*
                    overrides,
                    primitive,
                    depth_stencil,
                    multisample,
                    fragment,
                    multiview
                } : #pipeline_key,
                cache: Option<&wgpu::PipelineCache>,
        ) -> wgpu::RenderPipeline {
            let device = &self.device;
            let module = &self.shader_module;
            let constants = overrides.constants();
            let compilation_options = wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            };
            let (fragment_entry, targets) = fragment.entry_point_and_targets();
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&self.layout),
                    vertex: wgpu::VertexState {
                            module,
                            entry_point: #entry_name,
                            compilation_options: compilation_options.clone(),
                            buffers: &[#(#vertex_buffer_layouts),*],
                    },
                    primitive,
                    depth_stencil,
                    multisample,
                    fragment: Some(
                        wgpu::FragmentState {
                            module,
                            entry_point: fragment_entry,
                            compilation_options,
                            targets,
                        }
                    ),
                    multiview,
                    cache,
            })
        }

        #[builder(finish_fn = get)]
        pub fn #function_name(
                &self,
                #(#[builder(start_fn)] #step_args: wgpu::VertexStepMode,)*
                #[builder(default)] overrides: OverrideConstants,
                #[builder(default)] primitive: wgpu::PrimitiveState,
                depth_stencil: Option<wgpu::DepthStencilState>,
                #[builder(default)] multisample: wgpu::MultisampleState,
                fragment: FragmentEntry,
                multiview: Option<std::num::NonZero<u32>>,
                cache: Option<&wgpu::PipelineCache>,
        ) -> std::sync::Arc<wgpu::RenderPipeline> {
            let key = #pipeline_key {
                #(#step_args,)*
                overrides,
                primitive,
                depth_stencil,
                multisample,
                fragment,
                multiview
            };
            self.#pipeline_cache.lock().unwrap().entry(key).or_insert_with_key(
                |key| std::sync::Arc::new(self.#from_key_name(key.clone(), cache))
            ).clone()
        }
    };
    PipelineData {
        pipeline_cache,
        pipeline_key,
        pipeline_key_definition,
        pipeline_impl_definitions,
    }
}

// TODO: Figure out the most ergonomic place to output this.
fn workgroup_size(e: &naga::EntryPoint) -> TokenStream {
    let name = Ident::new(
        &format!("{}_WORKGROUP_SIZE", e.name.to_uppercase()),
        Span::call_site(),
    );
    let [x, y, z] = e
        .workgroup_size
        .map(|s| Literal::usize_unsuffixed(s as usize));
    quote!(pub const #name: [u32; 3] = [#x, #y, #z];)
}

fn define_compute_pipeline_key(entry_name: &str) -> (TokenStream, Ident) {
    let name = Ident::new(&format!("PipelineKey_{}", entry_name), Span::call_site());
    (
        quote! {
            #[derive(Clone, PartialEq, Eq, Hash)]
            struct #name {
                overrides: OverrideConstants,
            }
        },
        name,
    )
}

fn define_create_compute_pipeline(entry: &naga::EntryPoint) -> PipelineData {
    let entry_name = &entry.name;
    let (pipeline_key_definition, pipeline_key) = define_compute_pipeline_key(entry_name);

    let pipeline_cache = Ident::new(&format!("{}_pipelines", entry_name), Span::call_site());
    let function_name = Ident::new(&format!("{}_pipeline", entry_name), Span::call_site());
    let from_key_name = Ident::new(&format!("{}_from_key", entry_name), Span::call_site());

    let pipeline_impl_definitions = quote! {
        fn #from_key_name(
            &self,
            #pipeline_key { overrides } : #pipeline_key,
            cache: Option<&wgpu::PipelineCache>,
        ) -> wgpu::ComputePipeline {
            let device = &self.device;
            let module = &self.shader_module;
            let constants = overrides.constants();
            let compilation_options = wgpu::PipelineCompilationOptions {
                    constants: &constants,
                    ..Default::default()
            };
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&self.layout),
                    module,
                    entry_point: #entry_name,
                    compilation_options,
                    cache,
            })
        }

        #[builder(finish_fn = get)]
        pub fn #function_name(
            &self,
            #[builder(default)] overrides: OverrideConstants,
            cache: Option<&wgpu::PipelineCache>,
        ) -> std::sync::Arc<wgpu::ComputePipeline> {
            let key = #pipeline_key { overrides };
            self.#pipeline_cache.lock().unwrap().entry(key).or_insert_with_key(
                |key| std::sync::Arc::new(self.#from_key_name(key.clone(), cache))
            ).clone()
        }
    };
    PipelineData {
        pipeline_cache,
        pipeline_key,
        pipeline_key_definition,
        pipeline_impl_definitions,
    }
}

pub fn define_pipeline_layout(module: &naga::Module, bind_groups: &[BindGroup]) -> TokenStream {
    let (pipeline_datas, pipeline_results): (Vec<_>, Vec<_>) = module
        .entry_points
        .iter()
        .filter_map(|e| match e.stage {
            naga::ShaderStage::Vertex => Some((
                define_create_render_pipeline(module, e),
                quote!(wgpu::RenderPipeline),
            )),
            naga::ShaderStage::Compute => Some((
                define_create_compute_pipeline(e),
                quote!(wgpu::ComputePipeline),
            )),
            _ => None,
        })
        .unzip();

    let bind_group_layout_types: Vec<_> = bind_groups.iter().map(|g| &g.layout_type).collect();
    let bind_group_layouts_type = quote!((#(#bind_group_layout_types,)*));

    let pipeline_cache_field_names: Vec<_> = pipeline_datas
        .iter()
        .map(|data| data.pipeline_cache.clone())
        .collect();
    let pipeline_cache_field_types = pipeline_datas.iter().zip(pipeline_results.iter()).map(|(PipelineData { pipeline_key, ..}, result)|
        quote!(std::sync::Mutex<std::collections::HashMap<#pipeline_key, std::sync::Arc<#result>>>)
    );

    let pipeline_key_definitions = pipeline_datas
        .iter()
        .map(|data| &data.pipeline_key_definition);
    let pipeline_impl_definitions = pipeline_datas
        .iter()
        .map(|data| &data.pipeline_impl_definitions);

    quote! {
        pub struct PipelineLayout {
            device: std::sync::Arc<wgpu::Device>,
            shader_module: std::sync::Arc<wgpu::ShaderModule>,
            layout: wgpu::PipelineLayout,
            bind_group_layouts: #bind_group_layouts_type,
            #(#pipeline_cache_field_names: #pipeline_cache_field_types,)*
        }

        impl std::ops::Deref for PipelineLayout {
            type Target = wgpu::PipelineLayout;
            fn deref(&self) -> &Self::Target {
                &self.layout
            }
        }

        #(#pipeline_key_definitions)*

        #[bon::bon]
        impl PipelineLayout {
            pub fn new(
                device: std::sync::Arc<wgpu::Device>,
                shader_module: std::sync::Arc<wgpu::ShaderModule>,
                layout: wgpu::PipelineLayout,
                bind_group_layouts: #bind_group_layouts_type) -> Self {
                Self {
                    device, shader_module, layout, bind_group_layouts,
                    #(#pipeline_cache_field_names: Default::default(),)*
                }
            }

            pub fn bind_group_layouts(&self) -> &#bind_group_layouts_type {
                            &self.bind_group_layouts
            }

            #(#pipeline_impl_definitions)*
        }
    }
}
