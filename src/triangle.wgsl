[[stage(vertex)]]
fn vs_main(
	[[location(0)]] in_position: vec2<f32>,
) -> [[builtin(position)]] vec4<f32> {
	return vec4<f32>(in_position.x, in_position.y, 1.0, 1.0);
}

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> { 
	return vec4<f32>(1.0, 1.0, 1.0, 0.2);
}
