struct InstanceInputX_naga_oil_mod_XMNUGC4TUL5ZGKYLEX {
	@location(0) instance_result: vec4<f32>,
};

@vertex
fn vs_main(
	instance: InstanceInputX_naga_oil_mod_XMNUGC4TUL5ZGKYLEX,
) -> @builtin(position) vec4<f32> {
	return instance.instance_result;
}