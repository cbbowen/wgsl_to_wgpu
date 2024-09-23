use crate::bindgroup::BindGroup;
use proc_macro2::{Span, TokenStream};
use quote::quote;

fn define_create_pipeline_layout(
    bind_groups: &[BindGroup],
    all_bind_group_args: &[&syn::BareFnArg],
    push_constant_range: Option<TokenStream>,
) -> TokenStream {
    let bind_group_layouts: Vec<_> = bind_groups
        .iter()
        .map(|BindGroup { new, new_args, .. }| {
            let parameters: Vec<_> = new_args
                .iter()
                .map(|arg| &arg.name.as_ref().unwrap().0)
                .collect();
            quote!(#new(device.clone(), #(#parameters),*))
        })
        .collect();
    let bind_group_indices: Vec<_> = (0..bind_group_layouts.len() as u32)
        .map(|index| {
            syn::Member::Unnamed(syn::Index {
                index,
                span: Span::call_site(),
            })
        })
        .collect();
    let all_bind_group_arg_names: Vec<_> = all_bind_group_args
        .iter()
        .map(|arg| &arg.name.as_ref().unwrap().0)
        .collect();
    quote! {
        fn create_pipeline_layout(
            &self,
            PipelineLayoutKey { #(#all_bind_group_arg_names,)* }: PipelineLayoutKey,
        ) -> PipelineLayout {
            let device = self.device.clone();
            let bind_group_layouts = (#(#bind_group_layouts,)*);
            let layout = device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[#(&bind_group_layouts.#bind_group_indices),*],
                            push_constant_ranges: &[#push_constant_range],
            });
            let shader_module = self.shader_module.clone();
            PipelineLayout::new(device, shader_module, layout, bind_group_layouts)
        }

        #[builder(finish_fn = get)]
        pub fn pipeline_layout(
            &self,
            #(#all_bind_group_args,)*
        ) -> std::sync::Arc<PipelineLayout> {
            let key = PipelineLayoutKey { #(#all_bind_group_arg_names,)* };
            self.pipeline_layout_cache.lock().unwrap().entry(key).or_insert_with_key(
                |key| std::sync::Arc::new(self.create_pipeline_layout(key.clone()))
            ).clone()
        }
    }
}

pub fn define_shader(
    module: &naga::Module,
    bind_groups: &[BindGroup],
    push_constant_range: Option<TokenStream>,
) -> TokenStream {
    let mut validator = naga::valid::Validator::new(
        // TODO: We should probably make this part of the input options.
        naga::valid::ValidationFlags::empty(),
        // TODO: We should probably make this part of the input options.
        naga::valid::Capabilities::all(),
    );
    let module_info = validator.validate(module).unwrap();
    let wgsl_source = naga::back::wgsl::write_string(
        module,
        &module_info,
        // Without this, Naga changes `let A: f32 = 0f;` to `const: A = 0f;` which it then doesn't think is valid.
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap();

    let all_bind_group_args: Vec<_> = bind_groups.iter().flat_map(|g| &g.new_args).collect();
    let create_pipeline_layout =
        define_create_pipeline_layout(bind_groups, &all_bind_group_args, push_constant_range);
    let pipeline_layout_key_fields =
        all_bind_group_args
            .iter()
            .map(|syn::BareFnArg { name, ty, .. }| {
                let name = &name.as_ref().unwrap().0;
                quote!(#name: #ty)
            });

    quote! {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct PipelineLayoutKey {
            #(#pipeline_layout_key_fields,)*
        }

        pub struct Shader {
            device: std::sync::Arc<wgpu::Device>,
            shader_module: std::sync::Arc<wgpu::ShaderModule>,
            pipeline_layout_cache:
                std::sync::Mutex<
                    std::collections::HashMap<
                        PipelineLayoutKey,
                        std::sync::Arc<PipelineLayout>>>,
        }

        impl std::ops::Deref for Shader {
            type Target = wgpu::ShaderModule;
            fn deref(&self) -> &Self::Target {
                &self.shader_module
            }
        }

        #[bon::bon]
        impl Shader {
            pub const SOURCE: &'static str = #wgsl_source;

            pub fn new(device: std::sync::Arc<wgpu::Device>) -> Self {
                let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(Self::SOURCE)),
                });
                let shader_module = std::sync::Arc::new(shader_module);
                Self {
                    device,
                    shader_module,
                    pipeline_layout_cache: Default::default(),
                }
            }

            #create_pipeline_layout
        }
    }
}
