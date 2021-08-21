struct VertexStageOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[block]]
struct Radar {
    view_dir: vec2<f32>;
    position: vec2<f32>;
    fov: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
	[[location(0)]] in_position: vec2<f32>,
	[[location(1)]] in_tex_coords: vec2<f32>,
) -> VertexStageOutput {
	var out: VertexStageOutput;

	out.position = vec4<f32>(
		in_position,
		1.0,
		1.0
	);

	return out;
}

[[group(0), binding(0)]]
var<uniform> radar: Radar;

[[stage(fragment)]]
fn fs_main(
	in: VertexStageOutput
) -> [[location(0)]] vec4<f32> { 
	let at_radar = vec2<f32>(in.position.xy - radar.position);

	let angle_from_view_direction = acos(dot(radar.view_dir, at_radar) / (length(radar.view_dir) * length(at_radar)));
	let fov = radar.fov.x;

	//if (abs(angle_from_view_direction) < (fov / 2.0)) {
	if (in.position.x > 0.5) {
		//return vec4<f32>(0.0, 1.0, 0.9, 1.0);
		return vec4<f32>(1.0, 0.1, 0.3, 0.3);
	} else {
		return vec4<f32>(0.0, 1.0, 0.0, 1.0);
	}
}
