use crate::{
    quote_shader_stages, wgsl::buffer_binding_type, CreateModuleError,
};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::BTreeMap;
use syn::Ident;

pub struct GroupData<'a> {
    pub bindings: Vec<GroupBinding<'a>>,
}

pub struct GroupBinding<'a> {
    pub name: Option<String>,
    pub binding_index: u32,
    pub binding_type: &'a naga::Type,
    pub address_space: naga::AddressSpace,
}

pub struct BindGroup {
    pub layout_type: Ident,
    pub new: syn::Path,
    pub new_args: Vec<syn::BareFnArg>,
}

// TODO: Take an iterator instead?
pub fn bind_groups_module(
    bind_group_data: &BTreeMap<u32, GroupData>,
    shader_stages: wgpu::ShaderStages,
) -> (TokenStream, Vec<BindGroup>) {
    let (bind_group_layouts, bind_groups): (Vec<_>, Vec<_>) = bind_group_data
        .iter()
        .map(|(group_no, group)| {
            bind_group_layout(
                *group_no,
                group,
                shader_stages,
            )
        })
        .unzip();

    // Create a module to avoid name conflicts with user structs.
    (
        quote! {
            #(#bind_group_layouts)*
        },
        bind_groups,
    )
}

fn bind_group_layout_new(
    group: &GroupData,
    shader_stages: wgpu::ShaderStages,
) -> (TokenStream, Vec<syn::BareFnArg>) {
    let (entries, args): (Vec<_>, Vec<_>) = group
        .bindings
        .iter()
        .map(|binding| {
            bind_group_layout_entry(binding.name.as_ref().unwrap(), binding, shader_stages)
        })
        .unzip();
    let args: Vec<_> = args.into_iter().flatten().collect();
    let args_without_attrs = args.iter().map(|syn::BareFnArg { name, ty, .. }| {
        let name = &name.as_ref().unwrap().0;
        quote!(#name: #ty)
    });
    (
        quote! {
            pub fn new(
                device: std::sync::Arc<wgpu::Device>,
                #(#args_without_attrs),*
            ) -> Self {
                let layout = device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &[#(#entries),*],
                    }
                );
                Self { device, layout }
            }
        },
        args,
    )
}

fn bind_group_layout_create_bind_group(group_name: &Ident, group: &GroupData) -> TokenStream {
    let (args, entries): (Vec<_>, Vec<_>) = group
        .bindings
        .iter()
        .map(|binding| {
            let binding_index = Literal::usize_unsuffixed(binding.binding_index as usize);
            let binding_name = binding.name.as_ref().unwrap();
            let name = Ident::new(binding.name.as_ref().unwrap(), Span::call_site());
            let (arg, resource) = match binding.binding_type.inner {
                naga::TypeInner::Struct { .. }
                | naga::TypeInner::Array { .. }
                | naga::TypeInner::Scalar { .. }
                | naga::TypeInner::Vector { .. }
                | naga::TypeInner::Matrix { .. } => {
                    (
                        quote!(#name: wgpu::BufferBinding<'_>),
                        quote!(wgpu::BindingResource::Buffer(#name))
                    )
                }
                naga::TypeInner::Image { .. } => {
                    (
                        quote!(#name: &wgpu::TextureView),
                        quote!(wgpu::BindingResource::TextureView(#name))
                    )
                }
                naga::TypeInner::Sampler { .. } => {
                    (
                        quote!(#name: &wgpu::Sampler),
                        quote!(wgpu::BindingResource::Sampler(#name))
                    )
                }
                // TODO: Better error handling.
                ref inner => panic!(
                    "Failed to generate BindingType for `{inner:?}` of '{binding_name}' at index {binding_index}.",
                ),
            };
            (
                arg,
                quote!{
                    wgpu::BindGroupEntry {
                        binding: #binding_index,
                        resource: #resource,
                    }
                }
            )
        }).unzip();
    quote! {
        #[builder(finish_fn = create)]
        pub fn bind_group(
            &self,
            #(#args),*
        ) -> #group_name {
            let bind_group = self.device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &self.layout,
                    entries: &[
                        #(#entries),*
                    ],
                    label: None,
                }
            );
            #group_name(bind_group)
        }
    }
}

fn bind_group_layout(
    group_no: u32,
    group: &GroupData,
    shader_stages: wgpu::ShaderStages,
) -> (TokenStream, BindGroup) {
    let layout_name = Ident::new(&format!("BindGroupLayout{group_no}"), Span::call_site());
    let group_name = Ident::new(&format!("BindGroup{group_no}"), Span::call_site());
    let (new_def, new_args) = bind_group_layout_new(group, shader_stages);

    let create_bind_group = bind_group_layout_create_bind_group(&group_name, group);
    let new = syn::parse2(quote!(#layout_name::new)).unwrap();

    (
        quote! {
            #[derive(Debug)]
            pub struct #layout_name {
                device: std::sync::Arc<wgpu::Device>,
                layout: wgpu::BindGroupLayout,
            }

            impl std::ops::Deref for #layout_name {
                type Target = wgpu::BindGroupLayout;
                fn deref(&self) -> &Self::Target {
                    &self.layout
                }
            }

            pub struct #group_name(wgpu::BindGroup);

            impl std::ops::Deref for #group_name {
                type Target = wgpu::BindGroup;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            
            impl #group_name {
                pub fn set(&self, pass: &mut wgpu::RenderPass) {
                    pass.set_bind_group(#group_no, self, &[]);
                }

                pub fn set_compute(&self, pass: &mut wgpu::ComputePass) {
                    pass.set_bind_group(#group_no, self, &[]);
                }
            }

            #[bon::bon]
            impl #layout_name {
                #new_def
                #create_bind_group
            }
        },
        BindGroup {
            layout_type: layout_name,
            new,
            new_args,
        },
    )
}

fn bind_group_layout_entry(
    name: &str,
    binding: &GroupBinding,
    shader_stages: wgpu::ShaderStages,
) -> (TokenStream, Vec<syn::BareFnArg>) {
    // TODO: Assume storage is only used for compute?
    // TODO: Support just vertex or fragment?
    // TODO: Visible from all stages?
    let stages = quote_shader_stages(shader_stages);

    let binding_index = Literal::usize_unsuffixed(binding.binding_index as usize);
    let buffer_binding_type = buffer_binding_type(binding.address_space);
    let mut args = Vec::new();

    // TODO: Support more types.
    let binding_type = match binding.binding_type.inner {
        naga::TypeInner::Struct { .. }
        | naga::TypeInner::Array { .. }
        | naga::TypeInner::Scalar { .. }
        | naga::TypeInner::Vector { .. }
        | naga::TypeInner::Matrix { .. } => {
            quote!(wgpu::BindingType::Buffer {
                ty: #buffer_binding_type,
                has_dynamic_offset: false,
                min_binding_size: None,
            })
        }
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
            ..
        } => {
            let view_dim = match (dim, arrayed) {
                (naga::ImageDimension::D1, false) => quote!(wgpu::TextureViewDimension::D1),
                (naga::ImageDimension::D2, false) => quote!(wgpu::TextureViewDimension::D2),
                (naga::ImageDimension::D2, true) => quote!(wgpu::TextureViewDimension::D2Array),
                (naga::ImageDimension::D3, false) => quote!(wgpu::TextureViewDimension::D3),
                (naga::ImageDimension::Cube, false) => quote!(wgpu::TextureViewDimension::Cube),
                (naga::ImageDimension::Cube, true) => quote!(wgpu::TextureViewDimension::CubeArray),
                _ => panic!("Unsupported image dimension {dim:?}, arrayed = {arrayed}"),
            };

            match class {
                naga::ImageClass::Sampled { kind, multi } => {
                    let sample_type = match kind {
                        naga::ScalarKind::Sint => quote!(wgpu::TextureSampleType::Sint),
                        naga::ScalarKind::Uint => quote!(wgpu::TextureSampleType::Uint),
                        naga::ScalarKind::Float => {
                            let filterable =
                                Ident::new(&format!("{name}_filterable"), Span::call_site());
                            args.push(quote!( #[builder(default = true)] #filterable: bool ));
                            quote!(wgpu::TextureSampleType::Float { filterable: #filterable })
                        }
                        _ => todo!(),
                    };
                    quote!(wgpu::BindingType::Texture {
                        sample_type: #sample_type,
                        view_dimension: #view_dim,
                        multisampled: #multi,
                    })
                }
                naga::ImageClass::Depth { multi } => {
                    quote!(wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: #view_dim,
                        multisampled: #multi,
                    })
                }
                naga::ImageClass::Storage { format, access } => {
                    // TODO: Will the debug implementation always work with the macro?
                    // Assume texture format variants are the same as storage formats.
                    let format = syn::Ident::new(&format!("{format:?}"), Span::call_site());
                    let storage_access = storage_access(access);

                    quote!(wgpu::BindingType::StorageTexture {
                        access: #storage_access,
                        format: wgpu::TextureFormat::#format,
                        view_dimension: #view_dim,
                    })
                }
            }
        }
        naga::TypeInner::Sampler { comparison } => {
            let sampler_type = if comparison {
                quote!(wgpu::SamplerBindingType::Comparison)
            } else {
                let sampler_type = Ident::new(&format!("{name}_filtering"), Span::call_site());
                args.push(quote! { #[builder(default = wgpu::SamplerBindingType::Filtering)] #sampler_type: wgpu::SamplerBindingType });
                sampler_type.to_token_stream()
            };
            quote!(wgpu::BindingType::Sampler(#sampler_type))
        }
        // TODO: Better error handling.
        ref inner => {
            panic!("Failed to generate BindingType for `{inner:?}` at index {binding_index}.",)
        }
    };

    let args = args
        .into_iter()
        .map(|tokens| syn::parse2(tokens).unwrap())
        .collect();

    (
        quote! {
            wgpu::BindGroupLayoutEntry {
                binding: #binding_index,
                visibility: #stages,
                ty: #binding_type,
                count: None,
            }
        },
        args,
    )
}

fn storage_access(access: naga::StorageAccess) -> TokenStream {
    let is_read = access.contains(naga::StorageAccess::LOAD);
    let is_write = access.contains(naga::StorageAccess::STORE);
    match (is_read, is_write) {
        (true, true) => quote!(wgpu::StorageTextureAccess::ReadWrite),
        (true, false) => quote!(wgpu::StorageTextureAccess::ReadOnly),
        (false, true) => quote!(wgpu::StorageTextureAccess::WriteOnly),
        _ => todo!(), // shouldn't be possible
    }
}

pub fn get_bind_group_data(
    module: &naga::Module,
) -> Result<BTreeMap<u32, GroupData>, CreateModuleError> {
    // Use a BTree to sort type and field names by group index.
    // This isn't strictly necessary but makes the generated code cleaner.
    let mut groups = BTreeMap::new();

    for global_handle in module.global_variables.iter() {
        let global = &module.global_variables[global_handle.0];
        if let Some(binding) = &global.binding {
            let group = groups.entry(binding.group).or_insert(GroupData {
                bindings: Vec::new(),
            });
            let binding_type = &module.types[module.global_variables[global_handle.0].ty];

            let group_binding = GroupBinding {
                name: global.name.clone(),
                binding_index: binding.binding,
                binding_type,
                address_space: global.space,
            };
            // Repeated bindings will probably cause a compile error.
            // We'll still check for it here just in case.
            if group
                .bindings
                .iter()
                .any(|g| g.binding_index == binding.binding)
            {
                return Err(CreateModuleError::DuplicateBinding {
                    binding: binding.binding,
                });
            }
            group.bindings.push(group_binding);
        }
    }

    // wgpu expects bind groups to be consecutive starting from 0.
    if groups.keys().map(|i| *i as usize).eq(0..groups.len()) {
        Ok(groups)
    } else {
        Err(CreateModuleError::NonConsecutiveBindGroups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_tokens_eq;
    use indoc::indoc;

    #[test]
    fn bind_group_data_consecutive_bind_groups() {
        let source = indoc! {r#"
            @group(0) @binding(0) var<uniform> a: vec4<f32>;
            @group(1) @binding(0) var<uniform> b: vec4<f32>;
            @group(2) @binding(0) var<uniform> c: vec4<f32>;

            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();
        assert_eq!(3, get_bind_group_data(&module).unwrap().len());
    }

    #[test]
    fn bind_group_data_first_group_not_zero() {
        let source = indoc! {r#"
            @group(1) @binding(0) var<uniform> a: vec4<f32>;

            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();
        assert!(matches!(
            get_bind_group_data(&module),
            Err(CreateModuleError::NonConsecutiveBindGroups)
        ));
    }

    #[test]
    fn bind_group_data_non_consecutive_bind_groups() {
        let source = indoc! {r#"
            @group(0) @binding(0) var<uniform> a: vec4<f32>;
            @group(1) @binding(0) var<uniform> b: vec4<f32>;
            @group(3) @binding(0) var<uniform> c: vec4<f32>;

            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();
        assert!(matches!(
            get_bind_group_data(&module),
            Err(CreateModuleError::NonConsecutiveBindGroups)
        ));
    }

    fn test_bind_groups(wgsl: &str, rust: &str, stages: wgpu::ShaderStages) {
        let module = naga::front::wgsl::parse_str(wgsl).unwrap();
        let bind_group_data = get_bind_group_data(&module).unwrap();
        let (actual, _) = bind_groups_module(&bind_group_data, stages);

        assert_tokens_eq!(rust.parse().unwrap(), actual);
    }

    #[test]
    fn bind_groups_module_compute() {
        test_bind_groups(
            include_str!("data/bindgroup/compute.wgsl"),
            include_str!("data/bindgroup/compute.rs"),
            wgpu::ShaderStages::COMPUTE,
        );
    }

    #[test]
    fn bind_groups_module_vertex_fragment() {
        // Test different texture and sampler types.
        // TODO: Storage textures.
        test_bind_groups(
            include_str!("data/bindgroup/vertex_fragment.wgsl"),
            include_str!("data/bindgroup/vertex_fragment.rs"),
            wgpu::ShaderStages::VERTEX_FRAGMENT,
        );
    }

    #[test]
    fn bind_groups_module_vertex() {
        // The actual content of the structs doesn't matter.
        // We only care about the groups and bindings.
        test_bind_groups(
            include_str!("data/bindgroup/vertex.wgsl"),
            include_str!("data/bindgroup/vertex.rs"),
            wgpu::ShaderStages::VERTEX,
        );
    }

    #[test]
    fn bind_groups_module_fragment() {
        // The actual content of the structs doesn't matter.
        // We only care about the groups and bindings.
        test_bind_groups(
            include_str!("data/bindgroup/fragment.wgsl"),
            include_str!("data/bindgroup/fragment.rs"),
            wgpu::ShaderStages::FRAGMENT,
        );
    }
}
