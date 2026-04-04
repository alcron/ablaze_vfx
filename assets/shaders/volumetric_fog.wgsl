#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4f;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let world_normal = normalize(mesh.world_normal);
    let view_dir = normalize(view.world_position.xyz - mesh.world_position.xyz);
    let fresnel_mask = pow(1.0 - abs(dot(world_normal, view_dir)), 10.0);
    let top_mask = smoothstep(1.0, 0.75, mesh.uv.y);
    let bottom_mask = smoothstep(0.0, 0.25, mesh.uv.y);
    let mask = top_mask * bottom_mask * (1.0 - fresnel_mask);
    let result_color = vec4f(color.rgb, color.a * mask);

    return FragmentOutput(result_color);
}