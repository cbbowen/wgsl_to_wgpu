#[derive(Clone, Debug)]
pub struct BindGroupLayout0 {
    device: wgpu::Device,
    layout: wgpu::BindGroupLayout,
}
impl std::ops::Deref for BindGroupLayout0 {
    type Target = wgpu::BindGroupLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
#[derive(Clone, Debug)]
pub struct BindGroup0(wgpu::BindGroup);
impl std::ops::Deref for BindGroup0 {
    type Target = wgpu::BindGroup;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl BindGroup0 {
    pub fn set(&self, pass: &mut wgpu::RenderPass) {
        pass.set_bind_group(0u32, &**self, &[]);
    }
    pub fn set_compute(&self, pass: &mut wgpu::ComputePass) {
        pass.set_bind_group(0u32, &**self, &[]);
    }
}
#[bon::bon]
impl BindGroupLayout0 {
    pub fn new(
        device: wgpu::Device,
        color_texture3_filterable: bool,
        color_sampler_filtering: wgpu::SamplerBindingType,
        color_texture_msaa_filterable: bool,
        color_texture_array_2d_filterable: bool,
        color_texture_array_cube_filterable: bool,
    ) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Sint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: color_texture3_filterable,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(color_sampler_filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
                        format: wgpu::TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 7,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rg32Sint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 8,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba8Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 9,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: color_texture_msaa_filterable,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: true,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 10,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: true,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 11,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: color_texture_array_2d_filterable,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 12,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: color_texture_array_cube_filterable,
                        },
                        view_dimension: wgpu::TextureViewDimension::CubeArray,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
        Self { device, layout }
    }
    # [builder (finish_fn = create)]
    pub fn bind_group(
        &self,
        color_texture1: &wgpu::TextureView,
        color_texture2: &wgpu::TextureView,
        color_texture3: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
        color_sampler: &wgpu::Sampler,
        comparison_sampler: &wgpu::Sampler,
        storage_tex_read: &wgpu::TextureView,
        storage_tex_write: &wgpu::TextureView,
        storage_tex_read_write: &wgpu::TextureView,
        color_texture_msaa: &wgpu::TextureView,
        depth_texture_msaa: &wgpu::TextureView,
        color_texture_array_2d: &wgpu::TextureView,
        color_texture_array_cube: &wgpu::TextureView,
    ) -> BindGroup0 {
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(color_texture1),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(color_texture2),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(color_texture3),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(depth_texture),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(color_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(comparison_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(storage_tex_read),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::TextureView(storage_tex_write),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::TextureView(storage_tex_read_write),
                },
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: wgpu::BindingResource::TextureView(color_texture_msaa),
                },
                wgpu::BindGroupEntry {
                    binding: 10,
                    resource: wgpu::BindingResource::TextureView(depth_texture_msaa),
                },
                wgpu::BindGroupEntry {
                    binding: 11,
                    resource: wgpu::BindingResource::TextureView(color_texture_array_2d),
                },
                wgpu::BindGroupEntry {
                    binding: 12,
                    resource: wgpu::BindingResource::TextureView(color_texture_array_cube),
                },
            ],
            label: None,
        });
        BindGroup0(bind_group)
    }
}
#[derive(Clone, Debug)]
pub struct BindGroupLayout1 {
    device: wgpu::Device,
    layout: wgpu::BindGroupLayout,
}
impl std::ops::Deref for BindGroupLayout1 {
    type Target = wgpu::BindGroupLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
#[derive(Clone, Debug)]
pub struct BindGroup1(wgpu::BindGroup);
impl std::ops::Deref for BindGroup1 {
    type Target = wgpu::BindGroup;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl BindGroup1 {
    pub fn set(&self, pass: &mut wgpu::RenderPass) {
        pass.set_bind_group(1u32, &**self, &[]);
    }
    pub fn set_compute(&self, pass: &mut wgpu::ComputePass) {
        pass.set_bind_group(1u32, &**self, &[]);
    }
}
#[bon::bon]
impl BindGroupLayout1 {
    pub fn new(device: wgpu::Device) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        Self { device, layout }
    }
    # [builder (finish_fn = create)]
    pub fn bind_group(
        &self,
        transforms: wgpu::BufferBinding<'_>,
        scalar: wgpu::BufferBinding<'_>,
        vector: wgpu::BufferBinding<'_>,
        matrix: wgpu::BufferBinding<'_>,
    ) -> BindGroup1 {
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(transforms),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(scalar),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(vector),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(matrix),
                },
            ],
            label: None,
        });
        BindGroup1(bind_group)
    }
}
