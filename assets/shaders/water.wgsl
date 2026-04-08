#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::{globals, view, view_transmission_texture, view_transmission_sampler},
    forward_io::{Vertex, VertexOutput, FragmentOutput},
    prepass_utils::prepass_depth,
    view_transformations::depth_ndc_to_view_z,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> water_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> intersection_water_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> dark_water_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> darker_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> top_crest_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var noise_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var sparkles_noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var sparkles_noise_texture_sampler: sampler;

const PI: f32 = acos(-1.0);

// TODO Remove if unused
fn linear_to_polar(uv: vec2f, rotation: f32) -> vec2f {
    let centered = uv - vec2f(0.5);
    // let angle = atan2(centered.y, centered.x) / (2.0 * PI) + 0.5;
    // let angle = (PI + atan2(centered.x, centered.y)) / (2.0 * PI);
    let angle = fract((PI + atan2(centered.x, centered.y)) / (2.0 * PI) + 0.5 + rotation / (2.0 * PI));
    let radius = length(centered) * 2.0;

    return vec2f(angle, radius);
}

fn generate_polar_uv(uv: vec2f, uv_scale: f32, center_offset: vec2f, move_speed: f32) -> vec2f {
    let origin = center_offset * 0.5 + 0.5;
    let delta = uv - origin;
    let angle = (atan2(delta.y, delta.x) + PI) / (2.0 * PI);
    let radius = length(delta) / uv_scale;
    let polar_uv = vec2f(angle, radius * move_speed - globals.time * 0.1);

    return polar_uv;
}

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;

    var position = vertex.position;
    let model = mesh_functions::get_world_from_local(vertex.instance_index);

    let noise_origin_1 = vec2f(1.0, 1.0);
    let noise_origin_2 = vec2f(2.0, 1.5);
    let move_speed = 0.5;
    let uv_scale = 0.3;
    let noise_uv_1 = generate_polar_uv(vertex.uv, uv_scale, noise_origin_1, move_speed + 0.1);
    let noise_uv_2 = generate_polar_uv(vertex.uv, uv_scale, noise_origin_2, move_speed + 0.1);
    let noise_1 = textureSampleLevel(noise_texture, noise_texture_sampler, noise_uv_1, 0.0).r;
    let noise_2 = textureSampleLevel(noise_texture, noise_texture_sampler, noise_uv_2, 0.0).r;

    let circle_gradient = distance(vertex.uv, vec2(0.5));
    let circle = 1.0 - smoothstep(0.2, 0.5, circle_gradient);

    // let intencity = 0.35;
    let intencity = 0.45;
    let wave_noise = noise_1 * noise_2 * intencity;

    position += vertex.normal * wave_noise * circle;

    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    // Depth fade (soft intersection with scene geometry)
    let frag_depth = depth_ndc_to_view_z(mesh.position.z);
    let scene_depth = depth_ndc_to_view_z(prepass_depth(mesh.position, 0u));
    let depth_diff = frag_depth - scene_depth;
    let fade_distance = 0.35;
    let min_opacity = 0.83;
    let depth_fade = mix(min_opacity, 1.0, smoothstep(0.0, fade_distance, depth_diff));

    // TODO Fix duplication with vertex shader
    let circle_gradient = distance(mesh.uv, vec2(0.5));
    let circle = smoothstep(0.40, 0.5, circle_gradient);
    let noise_origin_1 = vec2f(1.0, 1.0);
    let noise_origin_2 = vec2f(2.0, 1.5);
    let move_speed = 0.5;
    let uv_scale = 0.3;

    let noise_uv_1 = generate_polar_uv(mesh.uv, uv_scale, noise_origin_1, move_speed + 0.1);
    let noise_uv_2 = generate_polar_uv(mesh.uv, uv_scale, noise_origin_2, move_speed + 0.1);
    let noise_1 = textureSample(noise_texture, noise_texture_sampler, noise_uv_1).r;
    let noise_2 = textureSample(noise_texture, noise_texture_sampler, noise_uv_2).r;
    let sparkles_noise = textureSample(sparkles_noise_texture, sparkles_noise_texture_sampler, mesh.uv * 20.0).r;

    let waves_mask = (noise_1 + noise_2) * 0.5;
    let dark_area_mask = 1.0 - smoothstep(0.0, 0.2, waves_mask);
    // TODO: Find better mask generation for water crests
    let light_area_mask = smoothstep(0.2, 0.5, waves_mask) * sparkles_noise;
    let base_with_crest_color = mix(dark_water_color.rgb, top_crest_color.rgb, waves_mask);
    let waves_color = mix(base_with_crest_color, dark_water_color.rgb, dark_area_mask);
    let waves_with_chrest_color = mix(waves_color, vec3f(1.0), light_area_mask);
    let color = mix(waves_with_chrest_color, darker_color.rgb, circle);

    return FragmentOutput(vec4f(color, depth_fade));
}