#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var noise_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> center_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> edge_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> color_intensity: f32;

fn hash(p: vec2f) -> vec2f {
    let k = vec2f(
        dot(p, vec2f(127.1, 311.7)),
        dot(p, vec2f(269.5, 183.3))
    );
    return fract(sin(k) * 43758.5453) * 2.0 - 1.0;
}

fn noise(st: vec2f) -> f32 {
    let i = floor(st);
    let f = fract(st);

    // Quintic interpolation (C2 continuous)
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);

    // Gradient dot products at four corners
    let a = dot(hash(i), f);
    let b = dot(hash(i + vec2f(1.0, 0.0)), f - vec2f(1.0, 0.0));
    let c = dot(hash(i + vec2f(0.0, 1.0)), f - vec2f(0.0, 1.0));
    let d = dot(hash(i + vec2f(1.0, 1.0)), f - vec2f(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y) * 0.5 + 0.5;
}

const OCTAVES: i32 = 3;

fn fbm(st_in: vec2f, lacunarity: f32) -> f32 {
    var st = st_in;
    var value = 0.0;
    var amplitude = 0.5;

    for (var i = 0; i < OCTAVES; i++) {
        value += amplitude * noise(st);
        st *= lacunarity;
        amplitude *= 0.5;
    }
    return value;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    // var st = mesh.uv * view.viewport.z / view.viewport.w;
    // let noise = pow(fbm(st * 3.0 + vec2f(0.0, globals.time), 3.0), 2.0);

    // let distortion_strength = 0.3;
    // let noise_val = fbm(mesh.uv * 5.0 + vec2f(0.0, globals.time), 3.0) - 0.5;
    // let distorted_uv = (mesh.uv + vec2f(noise_val) * distortion_strength * pow(1.0 - mesh.uv.y, 2.0));

    let speed = 0.25;
    let intensity = 0.2;

    let noise_alpha = textureSample(noise_texture, noise_texture_sampler, mesh.uv + vec2f(0.0, globals.time * speed)).r * (1.0 - mesh.uv.y);
    let distorted_uv = mesh.uv + vec2f((noise_alpha - 0.5) * intensity, 0.0);
    let flame_alpha = textureSample(texture, texture_sampler, distorted_uv).r;
    let flame_color = mix(edge_color.rgb, center_color.rgb, flame_alpha) * color_intensity;

    return FragmentOutput(vec4f(flame_color, flame_alpha));

}