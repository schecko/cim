#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var elevation_texture: texture_2d<f32>;
@group(2) @binding(2) var elevation_sampler: sampler;

@fragment
fn fragment(vert: VertexOutput) -> @location(0) vec4<f32>
{
	let texture_size = textureDimensions(elevation_texture, 0);
	let elevation_raw = textureLoad(elevation_texture, vec2i(vert.uv * vec2<f32>(texture_size.xy)), 0).r;

	let elevation = textureSample(elevation_texture, elevation_sampler, vert.uv).r;
    return color * elevation * f32(elevation_raw);
}
