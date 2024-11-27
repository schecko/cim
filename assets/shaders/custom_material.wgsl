@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var color_texture: texture_2d<f32>;
@group(2) @binding(2) var color_sampler: sampler;

@fragment
fn fragment_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32>
{
    // Sample the color from the texture using the UV coordinates
    let texture_color = textureSample(color_texture, color_sampler, uv);

    // Multiply the sampled texture color by the uniform color
    return texture_color * color;
}
