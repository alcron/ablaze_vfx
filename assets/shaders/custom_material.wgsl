#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> age_ratio: f32;

const PI: f32 = acos(-1.0);

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let texture = textureSample(texture, texture_sampler, mesh.uv);
    let alpha = texture.r * sin(age_ratio * PI);

    return FragmentOutput(vec4f(color.rgb, alpha));
}