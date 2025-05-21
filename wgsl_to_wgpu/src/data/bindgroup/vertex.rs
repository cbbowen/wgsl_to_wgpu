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
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        Self { device, layout }
    }
    # [builder (finish_fn = create)]
    pub fn bind_group(&self, transforms: wgpu::BufferBinding<'_>) -> BindGroup0 {
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(transforms),
            }],
            label: None,
        });
        BindGroup0(bind_group)
    }
}
