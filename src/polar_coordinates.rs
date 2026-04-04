use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor}, math::VectorSpace, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef
};

pub struct PolarCoordinatesPlugin;

impl Plugin for PolarCoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    noise_texture: Option<Handle<Image>>,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/polar_coordinates.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
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
                .from_asset("FX_SM_PolarCoordinate_FlatHemisphere.gltf"),
            ),
        ),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: Srgba::hex("#FF00FF").unwrap().with_alpha(1.0).into(),
            noise_texture: Some(asset_server.load_with_settings(
                "textures/T_Perlin_Noise_M.png",
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
        // Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_rotation_x(90.0_f32.to_radians()),
            scale: Vec3::splat(0.5),
        },
    ));
}
