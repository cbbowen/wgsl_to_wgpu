use pretty_assertions::assert_eq;

#[test]
fn vertex_entries() {
    // Check vertex entry points and builtin attribute handling.
    let actual = wgsl_to_wgpu::create_shader_module(
        include_str!("wgsl/vertex_entries.wgsl"),
        "shader.wgsl",
        wgsl_to_wgpu::WriteOptions {
            rustfmt: true,
            ..Default::default()
        },
    )
    .unwrap();

    assert_eq!(include_str!("output/vertex_entries.rs"), actual);
}

#[test]
fn vertex_entries_force_override_constants() {
    // Check vertex entry points and builtin attribute handling.
    let actual = wgsl_to_wgpu::create_shader_module(
        include_str!("wgsl/vertex_entries.wgsl"),
        "shader.wgsl",
        wgsl_to_wgpu::WriteOptions {
            rustfmt: true,
            force_override_constants: true,
            ..Default::default()
        },
    )
    .unwrap();

    assert_eq!(
        include_str!("output/vertex_entries_force_override_constants.rs"),
        actual
    );
}
