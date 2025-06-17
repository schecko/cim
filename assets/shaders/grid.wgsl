
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tint: vec4<f32>;
@group(2) @binding(1) var base_texture: texture_2d<f32>;
@group(2) @binding(2) var base_sampler: sampler;

@fragment
fn fragment(vert: VertexOutput) -> @location(0) vec4<f32>
{
	let tex = textureSample(base_texture, base_sampler, vert.uv);
	// let diff = abs(vert.uv - vec2(0.5, 0.5));
	// let distanceSq = dot(diff, diff);
	// let alpha = 1.0 - step(0.5 - distanceSq, 0.4);
	// return tint * vert.color * vec4(1.0, 1.0, 1.0, alpha);
	return tint * vert.color * vec4(tex.r, tex.r, tex.r, tex.r);
}
