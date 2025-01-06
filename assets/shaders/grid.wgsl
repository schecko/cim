
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tint: vec4<f32>;

@fragment
fn fragment(vert: VertexOutput) -> @location(0) vec4<f32>
{
    return tint * vert.color;
}
