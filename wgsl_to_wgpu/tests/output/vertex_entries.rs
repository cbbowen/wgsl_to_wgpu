#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Input0 {
    pub in0: [f32; 4],
    pub in1: [f32; 4],
    pub in2: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Input1 {
    pub in3: [f32; 4],
    pub in4: [f32; 4],
    pub in5: [f32; 4],
    pub in6: [u32; 4],
}
#[derive(Default)]
pub struct OverrideConstants {}
impl OverrideConstants {
    pub fn constants(&self) -> std::collections::HashMap<String, f64> {
        let entries = std::collections::HashMap::from([]);
        entries
    }
}
#[derive(Debug)]
pub struct BindGroupLayout<const I: u32> {
    device: std::sync::Arc<wgpu::Device>,
    layout: wgpu::BindGroupLayout,
}
impl Input0 {
    pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input0, in0) as u64,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input0, in1) as u64,
            shader_location: 1,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input0, in2) as u64,
            shader_location: 2,
        },
    ];
    pub const fn vertex_buffer_layout(
        step_mode: wgpu::VertexStepMode,
    ) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Input0>() as u64,
            step_mode,
            attributes: &Input0::VERTEX_ATTRIBUTES,
        }
    }
}
impl Input1 {
    pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 4] = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input1, in3) as u64,
            shader_location: 3,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input1, in4) as u64,
            shader_location: 4,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::offset_of!(Input1, in5) as u64,
            shader_location: 5,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Uint32x4,
            offset: std::mem::offset_of!(Input1, in6) as u64,
            shader_location: 6,
        },
    ];
    pub const fn vertex_buffer_layout(
        step_mode: wgpu::VertexStepMode,
    ) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Input1>() as u64,
            step_mode,
            attributes: &Input1::VERTEX_ATTRIBUTES,
        }
    }
}
pub enum FragmentState {}
impl FragmentState {
    pub fn entry_point_and_targets(&self) -> (&'static str, &[Option<wgpu::ColorTargetState>]) {
        match self {}
    }
}
pub struct Shader {
    device: std::sync::Arc<wgpu::Device>,
    shader_module: std::sync::Arc<wgpu::ShaderModule>,
}
impl Deref<wgpu::ShaderModule> for Shader {
    fn deref(&self) -> &wgpu::ShaderModule {
        &self.shader_module
    }
}
impl Shader {
    pub const SOURCE : & 'static str = "struct Input0_ {\n    @location(0) in0_: vec4<f32>,\n    @location(1) in1_: vec4<f32>,\n    @location(2) in2_: vec4<f32>,\n}\n\nstruct Input1_ {\n    @location(3) in3_: vec4<f32>,\n    @location(4) in4_: vec4<f32>,\n    @builtin(vertex_index) index: u32,\n    @location(5) in5_: vec4<f32>,\n    @location(6) @interpolate(flat) in6_: vec4<u32>,\n}\n\n@vertex \nfn vs_main_none() -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_single(in0_: Input0_) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_multiple(in0_1: Input0_, in1_: Input1_, @builtin(instance_index) in2_: u32, @location(7) in3_: vec4<f32>) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n" ;
    pub fn new(device: std::sync::Arc<wgpu::Device>) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: std::borrow::Cow::Borrowed(Self::SOURCE),
        });
        let shader_module = std::sync::Arc::new(shader_module);
        Self {
            device,
            shader_module,
        }
    }
    pub fn create_pipeline_layout(&self) -> PipelineLayout {
        let device = self.device.clone();
        let bind_group_layouts = ();
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let shader_module = self.shader_module.clone();
        PipelineLayout {
            device,
            shader_module,
            layout,
            bind_group_layouts,
        }
    }
}
pub struct PipelineLayout {
    device: std::sync::Arc<wgpu::Device>,
    shader_module: std::sync::Arc<wgpu::ShaderModule>,
    layout: wgpu::PipelineLayout,
    bind_group_layouts: bind_group_layouts_type,
}
impl Deref<wgpu::PipelineLayout> for PipelineLayout {
    fn deref(&self) -> &wgpu::PipelineLayout {
        &self.layout
    }
}
impl PipelineLayout {
    pub fn bind_group_layouts(&self) -> &() {
        &self.bind_group_layouts
    }
    pub fn create_vs_main_none_pipeline(
        &self,
        overrides: OverrideConstants,
        primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview: Option<NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let device = self.device;
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
                entry_point: "vs_main_none",
                compilation_options,
                buffers: &[],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: fragment_entry,
                compilation_options,
                targets,
            }),
            multiview,
            cache,
        })
    }
    pub fn create_vs_main_single_pipeline(
        &self,
        in0_step_mode: wgpu::VertexStepMode,
        overrides: OverrideConstants,
        primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview: Option<NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let device = self.device;
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
                entry_point: "vs_main_single",
                compilation_options,
                buffers: &[Input0::vertex_buffer_layout(in0_step_mode)],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: fragment_entry,
                compilation_options,
                targets,
            }),
            multiview,
            cache,
        })
    }
    pub fn create_vs_main_multiple_pipeline(
        &self,
        in0_step_mode: wgpu::VertexStepMode,
        in1_step_mode: wgpu::VertexStepMode,
        overrides: OverrideConstants,
        primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview: Option<NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let device = self.device;
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
                entry_point: "vs_main_multiple",
                compilation_options,
                buffers: &[
                    Input0::vertex_buffer_layout(in0_step_mode),
                    Input1::vertex_buffer_layout(in1_step_mode),
                ],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: fragment_entry,
                compilation_options,
                targets,
            }),
            multiview,
            cache,
        })
    }
}
