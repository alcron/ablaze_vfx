use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    water_color: LinearRgba,
    #[uniform(1)]
    intersection_water_color: LinearRgba,
    #[uniform(2)]
    dark_water_color: LinearRgba,
    #[uniform(3)]
    darker_color: LinearRgba,
    #[uniform(4)]
    top_crest_color: LinearRgba,
    #[texture(5)]
    #[sampler(6)]
    noise_texture: Option<Handle<Image>>,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// Right now used two difference normal oriented cones (like in lesson), but maybe in this examples it's better to use single one without disabling backface culling. It is possible that this effect will look good when there will be light source at base whitch hide mesh overlaps by higher intencity.
// TODO Make dust particles
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Transform {
            translation: Vec3::new(0.0, 0.2, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.5),
        },
    ));
    commands.spawn((
        Mesh3d(
            asset_server.load(
                GltfAssetLabel::Primitive {
                    mesh: 0,
                    primitive: 0,
                }
                .from_asset("FX_SM_Water_Disc.gltf"),
            ),
        ),
        MeshMaterial3d(materials.add(CustomMaterial {
            water_color: Srgba::hex("#00FDF3").unwrap().into(),
            intersection_water_color: Srgba::hex("#50B6C0").unwrap().into(),
            dark_water_color: Srgba::hex("#153B52").unwrap().into(),
            darker_color: Srgba::hex("#183351").unwrap().into(),
            top_crest_color: Srgba::hex("#00FFFF").unwrap().into(),
            noise_texture: Some(asset_server.load_with_settings(
                "textures/Water_NoisePerlin.png",
                |s: &mut ImageLoaderSettings| {
                    s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    });
                },
            )),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform {
            translation: Vec3::new(0.0, 0.35, 0.0),
            rotation: Quat::from_rotation_x(40.0_f32.to_radians()),
            scale: Vec3::splat(1.0),
        },
    ));
}
