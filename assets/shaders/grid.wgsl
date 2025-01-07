
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> tint: vec4<f32>;

@fragment
fn fragment(vert: VertexOutput) -> @location(0) vec4<f32>
{
	let diff = abs(vert.uv - vec2(0.5, 0.5));
	let distanceSq = dot(diff, diff);
	let alpha = 1.0 - step(0.5 - distanceSq, 0.4);
    return tint * vert.color * vec4(1.0, 1.0, 1.0, alpha);

	// let horizontal = abs(dot(vert.world_normal.xy, vec2(1.0, 0.0)));
	// let vertical = abs(dot(vert.world_normal.xy, vec2(0.0, 1.0)));
	// let alpha = saturate(horizontal + vertical);
    // return tint * vert.color * vec4(1.0, 1.0, 1.0, 1.0 - alpha );
    // return /*tint * vert.color * */vec4(vert.world_normal.xyz, 1.0);
}
