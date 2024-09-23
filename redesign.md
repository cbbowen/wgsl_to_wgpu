If we're entertaining large, breaking changes, what interface would I like?

```rs
// We don't need a uniform interface for bind groups, just a nice one.
pub struct BindGroupLayout<const I: usize> { ... }
impl<const I: usize> Deref<Target=wgpu::BindGroupLayout> for BindGroupLayout<I>;

#[bon]
impl BindGroupLayout<0> {
	pub fn new(
		texture_filterable: bool,
	) -> Self;

	#[builder(finish_fn = create)]
	pub fn bind_group(
		&self,
		texture: &wgpu::TextureView,
		sampler: &wgpu::Sampler,
	) -> wgpu::BindGroup;
}

#[derive(PartialEq, Eq, Hash)]
pub enum FragmentEntry {
	fs_main {
		targets: [Option<wgpu::ColorTargetState>; 1]
	},
}

pub struct BindGroup<const I: usize> { ... }
impl<const I: usize> Deref<Target=wgpu::BindGroup> for BindGroup<I>;

impl<const I: usize> BindGroup<I> {
	pub fn set_bind_group(&self, pass: &mut wgpu::RenderPass);
}

// It could be nice to have a generic `Shader` trait that these implement.
pub struct Shader { ... }
impl Deref<Target=wgpu::ShaderModule> for Shader;

#[bon]
impl Shader {
	pub fn new(device: Arc<wgpu::Device>) -> Self;
	
	#[builder(finish_fn = create)]
	pub fn pipeline_layout(
		#[builder(default = true)] texture_filterable: bool
	) -> PipelineLayout;
}

pub struct PipelineLayout { ... }
impl Deref<Target=wgpu::PipelineLayout> for PipelineLayout;

#[derive(PartialEq, Eq, Hash)]
struct PipelineKey {
	in_0_step_mode: wgpu::VertexStepMode,
	fragment: FragmentEntry,
	overrides: OverrideConstants,
	multisample: wgpu::MultisampleState,
	depth_stencil: Option<wgpu::DepthStencilState>,
	multiview: Option<NonZero<u32>>,
}

#[bon]
impl PipelineLayout {
	pub fn bind_group_layouts(&self) -> (&BindGroupLayout<0>,)

	fn vs_main_pipeline_impl(
		&self,
		key: PipelineKey,
		cache: Option<&wgpu::PipelineCache>,
	) -> VsMainPipeline;

	#[builder(finish_fn = get)]
	pub fn vs_main_pipeline(
		&self,
		#[builder(start_fn)] in_0_step_mode: wgpu::VertexStepMode,
		fragment: FragmentEntry,
		#[builder(default)] overrides: OverrideConstants,
		#[builder(default)] multisample: wgpu::MultisampleState,
		#[builder(default)] primitive: wgpu::PrimitiveState,
		depth_stencil: Option<wgpu::DepthStencilState>,
		multiview: Option<NonZero<u32>>,
		cache: Option<&wgpu::PipelineCache>,
	) -> Arc<VsMainPipeline>;

	// Compute pipeline.
	pub fn create_cs_main_pipeline(
		&self,
		constants: OverrideConstants,
		...
	) -> wgpu::ComputePipeline;
}

pub struct VsMainPipeline { ... }
impl Deref<wgpu::RenderPipeline> for RenderPipeline;

impl VsMainPipeline {
	fn in_0_layout(&self) -> wgpu::VertexBufferLayout<'_>;
}
```

Usage:
```rs
let shader = Shader::new(device);
let pipeline_layout = shader.create_pipeline_layout(false);
let pipeline = pipeline_layout.vs_main_pipeline()
	.fragment(FragmentEntry::fs_main([Some(color_target_state)]))
  .get();

let bind_group = pipeline_layout.create_bind_group(&texture_view, &sampler);
```