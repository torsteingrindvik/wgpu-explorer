let PI: f32 = 3.14159265359;

struct VertexStageOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[block]]
struct Radar {
    view_dir: vec2<f32>;
    position: vec2<f32>;
    fov: vec2<f32>; // Only first value is used (radians), second value for alignment
};

[[block]]
struct Resolution {
    size: vec2<f32>;
};

[[group(0), binding(0)]]
var<uniform> radar: Radar;

[[group(0), binding(1)]]
var<uniform> resolution: Resolution;

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

[[stage(fragment)]]
fn fs_main(
	in: VertexStageOutput
) -> [[location(0)]] vec4<f32> { 
	let position = vec2<f32>(in.position.xy - radar.position);
    let uv = vec2<f32>(position / resolution.size);
	let unit = (uv * 2.0) - vec2<f32>(1.0, 1.0);
	let at_radar = vec2<f32>(in.position.xy);

	let angle_from_view_direction = acos(dot(radar.view_dir, unit) / (length(radar.view_dir) * length(unit)));
	let fov = radar.fov.x;

	if (abs(angle_from_view_direction) < (fov / 2.0) && distance(unit, vec2<f32>(0.0, 0.0)) < 0.5) {
		return vec4<f32>(1.0, 0.2, 0.3, 0.3);
	} else {
		return vec4<f32>(0.2, 1.0, 0.0, 1.0);
	}
}
