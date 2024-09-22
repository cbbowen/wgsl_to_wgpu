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
pub const ENTRY_VS_MAIN_NONE: &str = "vs_main_none";
pub const ENTRY_VS_MAIN_SINGLE: &str = "vs_main_single";
pub const ENTRY_VS_MAIN_MULTIPLE: &str = "vs_main_multiple";
#[derive(Debug)]
pub struct VertexEntry<const N: usize> {
    pub entry_point: &'static str,
    pub buffers: [wgpu::VertexBufferLayout<'static>; N],
    pub constants: std::collections::HashMap<String, f64>,
}
pub fn vertex_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a VertexEntry<N>,
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point: entry.entry_point,
        buffers: &entry.buffers,
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &entry.constants,
            ..Default::default()
        },
    }
}
pub const NUM_VERTEX_INPUTS_VS_MAIN_NONE: usize = 0;
pub fn vs_main_none_entry() -> VertexEntry<NUM_VERTEX_INPUTS_VS_MAIN_NONE> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN_NONE,
        buffers: [],
        constants: Default::default(),
    }
}
pub const NUM_VERTEX_INPUTS_VS_MAIN_SINGLE: usize = 1;
pub fn vs_main_single_entry(
    input0: wgpu::VertexStepMode,
) -> VertexEntry<NUM_VERTEX_INPUTS_VS_MAIN_SINGLE> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN_SINGLE,
        buffers: [Input0::vertex_buffer_layout(input0)],
        constants: Default::default(),
    }
}
pub const NUM_VERTEX_INPUTS_VS_MAIN_MULTIPLE: usize = 2;
pub fn vs_main_multiple_entry(
    input0: wgpu::VertexStepMode,
    input1: wgpu::VertexStepMode,
) -> VertexEntry<NUM_VERTEX_INPUTS_VS_MAIN_MULTIPLE> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN_MULTIPLE,
        buffers: [
            Input0::vertex_buffer_layout(input0),
            Input1::vertex_buffer_layout(input1),
        ],
        constants: Default::default(),
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std :: borrow :: Cow :: Borrowed ("struct Input0_ {\n    @location(0) in0_: vec4<f32>,\n    @location(1) in1_: vec4<f32>,\n    @location(2) in2_: vec4<f32>,\n}\n\nstruct Input1_ {\n    @location(3) in3_: vec4<f32>,\n    @location(4) in4_: vec4<f32>,\n    @builtin(vertex_index) index: u32,\n    @location(5) in5_: vec4<f32>,\n    @location(6) @interpolate(flat) in6_: vec4<u32>,\n}\n\n@vertex \nfn vs_main_none() -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_single(in0_: Input0_) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n\n@vertex \nfn vs_main_multiple(in0_1: Input0_, in1_: Input1_, @builtin(instance_index) in2_: u32, @location(7) in3_: vec4<f32>) -> @builtin(position) vec4<f32> {\n    return vec4(0f);\n}\n") ;
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(source),
    })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
}
