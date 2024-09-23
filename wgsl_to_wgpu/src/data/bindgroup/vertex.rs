pub mod bind_groups {
    #[derive(Debug)]
    pub struct BindGroupLayout0(wgpu::BindGroupLayout);
    impl BindGroupLayout0 {
        pub fn new(device: &wgpu::Device) -> Self {
            Self(
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                }),
            )
        }
        pub fn create_bind_group(
            &self,
            device: &wgpu::Device,
            transforms: wgpu::BufferBinding<'_>,
        ) -> wgpu::BindGroup {
            let bind_group_layout = &self.0;
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(transforms),
                }],
                label: None,
            });
            bind_group
        }
    }
}
