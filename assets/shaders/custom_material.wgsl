#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tint: vec4<f32>;
// channel 1 - elevation
// channel 2 - unused
// channel 3 - unused
// channel 4 - unused
@group(2) @binding(1) var elevation_texture: texture_2d<f32>;
@group(2) @binding(2) var elevation_sampler: sampler;

@group(2) @binding(3) var color_palette: texture_1d<f32>;
@group(2) @binding(4) var palette_sampler: sampler;

@fragment
fn fragment(vert: VertexOutput) -> @location(0) vec4<f32>
{
	let elevation = textureSample(elevation_texture, elevation_sampler, vert.uv).r;
	let color = textureSample(color_palette, palette_sampler, elevation);
    return tint * color;
}
