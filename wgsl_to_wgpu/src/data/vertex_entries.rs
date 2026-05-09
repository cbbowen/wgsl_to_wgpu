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
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideConstants {}
impl OverrideConstants {
    pub fn constants(&self) -> Vec<(&'static str, f64)> {
        vec![]
    }
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FragmentEntry {}
impl FragmentEntry {
    pub fn entry_point_and_targets(&self) -> (&'static str, &[Option<wgpu::ColorTargetState>]) {
        unreachable!()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PipelineLayoutKey {}
pub struct Shader {
    device: wgpu::Device,
    shader_module: wgpu::ShaderModule,
    pipeline_layout_cache: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<PipelineLayoutKey, PipelineLayout>>,
    >,
}
impl std::ops::Deref for Shader {
    type Target = wgpu::ShaderModule;
    fn deref(&self) -> &Self::Target {
        &self.shader_module
    }
}
#[bon::bon]
impl Shader {
    pub const SOURCE : & 'static str = "struct Input0_ {\n    @location(0) @align(16) in0_: vec4<f32>,\n    @location(1) @align(16) in1_: vec4<f32>,\n    @location(2) @align(32) in2_: vec4<f32>,\n}\n\nstruct Input1_ {\n    @location(3) @align(16) in3_: vec4<f32>,\n    @location(4) @align(16) in4_: vec4<f32>,\n    @builtin(vertex_index) @align(32) index: u32,\n    @location(5) @align(16) in5_: vec4<f32>,\n    @location(6) @align(64) in6_: vec4<u32>,\n}\n\n@vertex \nfn vs_main_none() -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_single(in0_: Input0_) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_multiple(in0_1: Input0_, in1_: Input1_, @builtin(instance_index) in2_: u32, @location(7) in3_: vec4<f32>) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n" ;
    pub fn new(device: wgpu::Device) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(Self::SOURCE)),
        });
        Self {
            device,
            shader_module,
            pipeline_layout_cache: Default::default(),
        }
    }
    fn create_pipeline_layout(&self, PipelineLayoutKey {}: PipelineLayoutKey) -> PipelineLayout {
        let device = self.device.clone();
        let bind_group_layouts = ();
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0u32,
        });
        let shader_module = self.shader_module.clone();
        PipelineLayout::new(device, shader_module, layout, bind_group_layouts)
    }
    # [builder (finish_fn = get)]
    pub fn pipeline_layout(&self) -> PipelineLayout {
        let key = PipelineLayoutKey {};
        self.pipeline_layout_cache
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| self.create_pipeline_layout(key.clone()))
            .clone()
    }
}
#[derive(Clone, Debug)]
pub struct PipelineLayout {
    device: wgpu::Device,
    shader_module: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    bind_group_layouts: (),
    vs_main_none_pipelines: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<PipelineKey_vs_main_none, wgpu::RenderPipeline>>,
    >,
    vs_main_single_pipelines: std::sync::Arc<
        std::sync::Mutex<
            std::collections::HashMap<PipelineKey_vs_main_single, wgpu::RenderPipeline>,
        >,
    >,
    vs_main_multiple_pipelines: std::sync::Arc<
        std::sync::Mutex<
            std::collections::HashMap<PipelineKey_vs_main_multiple, wgpu::RenderPipeline>,
        >,
    >,
}
impl std::ops::Deref for PipelineLayout {
    type Target = wgpu::PipelineLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
struct PipelineKey_vs_main_none {
    overrides: OverrideConstants,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment: FragmentEntry,
    multiview_mask: Option<std::num::NonZero<u32>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
struct PipelineKey_vs_main_single {
    in0_step_mode: wgpu::VertexStepMode,
    overrides: OverrideConstants,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment: FragmentEntry,
    multiview_mask: Option<std::num::NonZero<u32>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
struct PipelineKey_vs_main_multiple {
    in0_step_mode: wgpu::VertexStepMode,
    in1_step_mode: wgpu::VertexStepMode,
    overrides: OverrideConstants,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment: FragmentEntry,
    multiview_mask: Option<std::num::NonZero<u32>>,
}
#[bon::bon]
impl PipelineLayout {
    pub fn new(
        device: wgpu::Device,
        shader_module: wgpu::ShaderModule,
        layout: wgpu::PipelineLayout,
        bind_group_layouts: (),
    ) -> Self {
        Self {
            device,
            shader_module,
            layout,
            bind_group_layouts,
            vs_main_none_pipelines: Default::default(),
            vs_main_single_pipelines: Default::default(),
            vs_main_multiple_pipelines: Default::default(),
        }
    }
    pub fn bind_group_layouts(&self) -> &() {
        &self.bind_group_layouts
    }
    fn pipeline_vs_main_none_from_key(
        &self,
        PipelineKey_vs_main_none {
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        }: PipelineKey_vs_main_none,
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
                entry_point: Some("vs_main_none"),
                compilation_options: compilation_options.clone(),
                buffers: &[],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some(fragment_entry),
                compilation_options,
                targets,
            }),
            multiview_mask,
            cache,
        })
    }
    # [builder (finish_fn = get)]
    pub fn vs_main_none_pipeline(
        &self,
        #[builder(default)] overrides: OverrideConstants,
        #[builder(default)] primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        #[builder(default)] multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview_mask: Option<std::num::NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let key = PipelineKey_vs_main_none {
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        };
        self.vs_main_none_pipelines
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| self.pipeline_vs_main_none_from_key(key.clone(), cache))
            .clone()
    }
    fn pipeline_vs_main_single_from_key(
        &self,
        PipelineKey_vs_main_single {
            in0_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        }: PipelineKey_vs_main_single,
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
                entry_point: Some("vs_main_single"),
                compilation_options: compilation_options.clone(),
                buffers: &[Some(Input0::vertex_buffer_layout(in0_step_mode))],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some(fragment_entry),
                compilation_options,
                targets,
            }),
            multiview_mask,
            cache,
        })
    }
    # [builder (finish_fn = get)]
    pub fn vs_main_single_pipeline(
        &self,
        #[builder(start_fn)] in0_step_mode: wgpu::VertexStepMode,
        #[builder(default)] overrides: OverrideConstants,
        #[builder(default)] primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        #[builder(default)] multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview_mask: Option<std::num::NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let key = PipelineKey_vs_main_single {
            in0_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        };
        self.vs_main_single_pipelines
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| self.pipeline_vs_main_single_from_key(key.clone(), cache))
            .clone()
    }
    fn pipeline_vs_main_multiple_from_key(
        &self,
        PipelineKey_vs_main_multiple {
            in0_step_mode,
            in1_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        }: PipelineKey_vs_main_multiple,
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
                entry_point: Some("vs_main_multiple"),
                compilation_options: compilation_options.clone(),
                buffers: &[
                    Some(Input0::vertex_buffer_layout(in0_step_mode)),
                    Some(Input1::vertex_buffer_layout(in1_step_mode)),
                ],
            },
            primitive,
            depth_stencil,
            multisample,
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some(fragment_entry),
                compilation_options,
                targets,
            }),
            multiview_mask,
            cache,
        })
    }
    # [builder (finish_fn = get)]
    pub fn vs_main_multiple_pipeline(
        &self,
        #[builder(start_fn)] in0_step_mode: wgpu::VertexStepMode,
        #[builder(start_fn)] in1_step_mode: wgpu::VertexStepMode,
        #[builder(default)] overrides: OverrideConstants,
        #[builder(default)] primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        #[builder(default)] multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview_mask: Option<std::num::NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let key = PipelineKey_vs_main_multiple {
            in0_step_mode,
            in1_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        };
        self.vs_main_multiple_pipelines
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| self.pipeline_vs_main_multiple_from_key(key.clone(), cache))
            .clone()
    }
}
