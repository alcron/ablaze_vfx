use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};

pub struct VolumetricFogPlugin;

impl Plugin for VolumetricFogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/volumetric_fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

// Right now used two difference normal oriented cones (like in lesson), but maybe in this examples it's better to use single one without disabling backface culling. It is possible that this effect will look good when there will be light source at base whitch hide mesh overlaps by higher intencity.
// TODO Make dust particles
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(
            asset_server.load(
                GltfAssetLabel::Primitive {
                    mesh: 0,
                    primitive: 0,
                }
                .from_asset("FX_SM_VolumetricLight_Cone.gltf"),
            ),
        ),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: Srgba::hex("#A5DDFF").unwrap().with_alpha(0.1).into(),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform {
            translation: Vec3::new(0.0, -0.35, 0.0),
            rotation: Quat::from_rotation_x(30.0_f32.to_radians()),
            scale: Vec3::splat(0.5),
        },
    ));
}
