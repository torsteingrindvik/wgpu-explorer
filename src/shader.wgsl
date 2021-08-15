struct VertexStageOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
	[[builtin(vertex_index)]] in_vertex_index: u32,
	[[location(0)]] in_position: vec2<f32>,
	[[location(1)]] in_tex_coords: vec2<f32>,
) -> VertexStageOutput {
	var out: VertexStageOutput;

	let idx = f32(in_vertex_index);
	let linear_min_to_max_by_idx = idx / 4.0;

	out.position = vec4<f32>(in_position, 1.0, 1.0);

	// out.color = vec3<f32>(
	// 	linear_min_to_max_by_idx,
	// 	linear_min_to_max_by_idx,
	// 	linear_min_to_max_by_idx
	// );

	out.tex_coords = in_tex_coords;

	return out;
}

// [[block]]
// struct MyColorWithAlpha {
// 	color: vec4<f32>;
// };

// [[block]]
// struct MyColor {
// 	color: vec3<f32>;
// };

// [[group(0), binding(0)]]
// var<uniform> my_color_with_alpha: MyColorWithAlpha;

// [[group(0), binding(1)]]
// var<uniform> my_color: MyColor;

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(
	in: VertexStageOutput
) -> [[location(0)]] vec4<f32> { 
	// return vec4<f32>(in.color, 0.1);
	return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
