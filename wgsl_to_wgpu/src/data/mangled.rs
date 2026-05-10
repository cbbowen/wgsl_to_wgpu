#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InstanceInput {
    pub instance_result: [f32; 4],
}
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideConstants {}
impl OverrideConstants {
    pub fn constants(&self) -> Vec<(&'static str, f64)> {
        vec![]
    }
}
impl InstanceInput {
    pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x4,
        offset: std::mem::offset_of!(InstanceInput, instance_result) as u64,
        shader_location: 0,
    }];
    pub const fn vertex_buffer_layout(
        step_mode: wgpu::VertexStepMode,
    ) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceInput>() as u64,
            step_mode,
            attributes: &InstanceInput::VERTEX_ATTRIBUTES,
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
    pub const SOURCE: & 'static str = "struct InstanceInputX_naga_oil_mod_XMNUGC4TUL5ZGKYLEX {\n    @location(0) @align(16) instance_result: vec4<f32>,\n}\n\n@vertex \nfn vs_main(instance: InstanceInputX_naga_oil_mod_XMNUGC4TUL5ZGKYLEX) -> @builtin(position) vec4<f32> {\n    return instance.instance_result;\n}\n" ;
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
    vs_main_pipelines: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<PipelineKey_vs_main, wgpu::RenderPipeline>>,
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
struct PipelineKey_vs_main {
    instance_step_mode: wgpu::VertexStepMode,
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
            vs_main_pipelines: Default::default(),
        }
    }
    pub fn bind_group_layouts(&self) -> &() {
        &self.bind_group_layouts
    }
    fn pipeline_vs_main_from_key(
        &self,
        PipelineKey_vs_main {
            instance_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        }: PipelineKey_vs_main,
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
                entry_point: Some("vs_main"),
                compilation_options: compilation_options.clone(),
                buffers: &[Some(InstanceInput::vertex_buffer_layout(
                    instance_step_mode,
                ))],
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
    pub fn vs_main_pipeline(
        &self,
        #[builder(start_fn)] instance_step_mode: wgpu::VertexStepMode,
        #[builder(default)] overrides: OverrideConstants,
        #[builder(default)] primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        #[builder(default)] multisample: wgpu::MultisampleState,
        fragment: FragmentEntry,
        multiview_mask: Option<std::num::NonZero<u32>>,
        cache: Option<&wgpu::PipelineCache>,
    ) -> wgpu::RenderPipeline {
        let key = PipelineKey_vs_main {
            instance_step_mode,
            overrides,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview_mask,
        };
        self.vs_main_pipelines
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| self.pipeline_vs_main_from_key(key.clone(), cache))
            .clone()
    }
}
