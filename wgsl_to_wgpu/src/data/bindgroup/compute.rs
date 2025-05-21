#[derive(Debug)]
pub struct BindGroupLayout0 {
    device: std::sync::Arc<wgpu::Device>,
    layout: wgpu::BindGroupLayout,
}
impl std::ops::Deref for BindGroupLayout0 {
    type Target = wgpu::BindGroupLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
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
    pub fn new(device: std::sync::Arc<wgpu::Device>) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
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
        src: wgpu::BufferBinding<'_>,
        vertex_weights: wgpu::BufferBinding<'_>,
        dst: wgpu::BufferBinding<'_>,
    ) -> BindGroup0 {
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(src),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(vertex_weights),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(dst),
                },
            ],
            label: None,
        });
        BindGroup0(bind_group)
    }
}
#[derive(Debug)]
pub struct BindGroupLayout1 {
    device: std::sync::Arc<wgpu::Device>,
    layout: wgpu::BindGroupLayout,
}
impl std::ops::Deref for BindGroupLayout1 {
    type Target = wgpu::BindGroupLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
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
    pub fn new(device: std::sync::Arc<wgpu::Device>, texture_filterable: bool) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: texture_filterable,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
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
        transforms: wgpu::BufferBinding<'_>,
        texture: &wgpu::TextureView,
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
                    resource: wgpu::BindingResource::TextureView(texture),
                },
            ],
            label: None,
        });
        BindGroup1(bind_group)
    }
}
