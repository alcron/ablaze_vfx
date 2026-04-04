#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    pbr_functions,
    mesh_view_bindings::globals,
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::view,
}

struct FresnelExtension {
    color: vec3f,
    thickness: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> fresnel_ext: FresnelExtension;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var noise_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Run standard PBR
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.alpha_cutoff = 0.0;
    var mesh_color = pbr_functions::apply_pbr_lighting(pbr_input);
    mesh_color = pbr_functions::main_pass_post_lighting_processing(pbr_input, mesh_color);

    // Add fresnel on top
    let V = normalize(view.world_position.xyz - in.world_position.xyz);
    let N = normalize(in.world_normal);
    // TODO Make smoothstep whitch will influence fresnel mask more linearly
    let power = mix(10.0, 0.01, fresnel_ext.thickness) * 2.0;
    let fresnel_alpha = pow(1.0 - saturate(dot(N, V)), power);
    let fresnel_color = vec4f(fresnel_ext.color, fresnel_alpha);

    let noise_tiling = 2.0;
    let noise_scrolling_speed = vec2f(-0.1, -0.1);

    let noise_mask = textureSample(noise_texture, noise_sampler, in.uv * noise_tiling + noise_scrolling_speed * globals.time).r;
    let color = mix(mesh_color, fresnel_color, clamp(noise_mask + fresnel_alpha, 0.0, 1.0) * fresnel_alpha);

    return FragmentOutput(color);
}