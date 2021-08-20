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

	out.position = vec4<f32>(in_position, 1.0, 1.0);
	out.tex_coords = in_tex_coords;

	return out;
}

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(
	in: VertexStageOutput
) -> [[location(0)]] vec4<f32> { 
	// return vec4<f32>(1.0, 1.0, 0.3, 0.1);
	return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
