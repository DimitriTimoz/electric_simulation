#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> emissive: f32;

fn random(seed: vec2<f32>) -> f32 {
    let dot_product = dot(seed, vec2<f32>(12.9898, 78.233));
    return fract(sin(dot_product) * 43758.5453123);
}

@fragment
fn fragment(
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    // Base lightning color
    let base_color = material_color.rgb;

    // Procedural flickering using noise and time
    let flicker = random(uv * vec2<f32>(10.0, 10.0) + globals.time);
    let flicker_intensity = step(0.7, flicker); // High intensity only at peaks

    // Emissive lightning effect
    let glow = emissive * flicker_intensity;

    // Combine color and glow
    let final_color = base_color * glow;

    return vec4(final_color, material_color.a); // Alpha from material_color
}
