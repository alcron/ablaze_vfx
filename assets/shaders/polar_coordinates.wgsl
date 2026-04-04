#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var noise_texture_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let speed = 0.5;
    let uv_scale_y = 0.08;
    let scaled_uv = vec2f(mesh.uv.x, mesh.uv.y * uv_scale_y);
    let noise_alpha = textureSample(noise_texture, noise_texture_sampler, scaled_uv - vec2f(0.0, 1.0) * globals.time * speed * uv_scale_y).r;
    // Because on hemisphere UV is not linear we have to adjust values of mask
    let inner_mask = smoothstep(0.1, 1.0, mesh.uv.y);
    let outer_mask = 1.0 - smoothstep(0.5, 0.75, mesh.uv.y);
    let alpha = inner_mask * outer_mask * noise_alpha * color.a;

    return FragmentOutput(vec4f(color.rgb, alpha));
}