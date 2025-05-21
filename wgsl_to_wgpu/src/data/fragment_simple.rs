#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideConstants {}
impl OverrideConstants {
    pub fn constants(&self) -> Vec<(&'static str, f64)> {
        [].into_iter().filter_map(|a| a).collect()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FragmentEntry {
    fs_main { targets: [Option<wgpu::ColorTargetState>; 0usize] },
}
impl FragmentEntry {
    pub fn entry_point_and_targets(
        &self,
    ) -> (&'static str, &[Option<wgpu::ColorTargetState>]) {
        match self {
            Self::fs_main { targets } => ("fs_main", targets),
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PipelineLayoutKey {}
pub struct Shader {
    device: std::sync::Arc<wgpu::Device>,
    shader_module: std::sync::Arc<wgpu::ShaderModule>,
    pipeline_layout_cache: std::sync::Mutex<
        std::collections::HashMap<PipelineLayoutKey, std::sync::Arc<PipelineLayout>>,
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
    pub const SOURCE: &'static str = "@fragment \nfn fs_main() {\n    return;\n}\n";
    pub fn new(device: std::sync::Arc<wgpu::Device>) -> Self {
        let shader_module = device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(
                    std::borrow::Cow::Borrowed(Self::SOURCE),
                ),
            });
        let shader_module = std::sync::Arc::new(shader_module);
        Self {
            device,
            shader_module,
            pipeline_layout_cache: Default::default(),
        }
    }
    fn create_pipeline_layout(
        &self,
        PipelineLayoutKey {}: PipelineLayoutKey,
    ) -> PipelineLayout {
        let device = self.device.clone();
        let bind_group_layouts = ();
        let layout = device
            .create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                },
            );
        let shader_module = self.shader_module.clone();
        PipelineLayout::new(device, shader_module, layout, bind_group_layouts)
    }
    #[builder(finish_fn = get)]
    pub fn pipeline_layout(&self) -> std::sync::Arc<PipelineLayout> {
        let key = PipelineLayoutKey {};
        self.pipeline_layout_cache
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with_key(|key| std::sync::Arc::new(
                self.create_pipeline_layout(key.clone()),
            ))
            .clone()
    }
}
pub struct PipelineLayout {
    device: std::sync::Arc<wgpu::Device>,
    shader_module: std::sync::Arc<wgpu::ShaderModule>,
    layout: wgpu::PipelineLayout,
    bind_group_layouts: (),
}
impl std::ops::Deref for PipelineLayout {
    type Target = wgpu::PipelineLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
#[bon::bon]
impl PipelineLayout {
    pub fn new(
        device: std::sync::Arc<wgpu::Device>,
        shader_module: std::sync::Arc<wgpu::ShaderModule>,
        layout: wgpu::PipelineLayout,
        bind_group_layouts: (),
    ) -> Self {
        Self {
            device,
            shader_module,
            layout,
            bind_group_layouts,
        }
    }
    pub fn bind_group_layouts(&self) -> &() {
        &self.bind_group_layouts
    }
}
