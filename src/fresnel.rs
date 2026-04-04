use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

type FresnelMaterial = ExtendedMaterial<StandardMaterial, FresnelExtension>;

pub struct FresnelPlugin;

impl Plugin for FresnelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FresnelMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct FresnelExtension {
    #[uniform(100)]
    pub color: Vec3,
    #[uniform(100)]
    pub thickness: f32,
    #[texture(101)]
    #[sampler(102)]
    noise_texture: Option<Handle<Image>>,
}

impl MaterialExtension for FresnelExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FresnelMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let color_rgb = LinearRgba::rgb(0.0, 0.0, 5.0);

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: FresnelExtension {
                // [0.0, 1.0] range
                thickness: 0.9,
                color: color_rgb.to_vec3(),
                noise_texture: Some(asset_server.load_with_settings(
                    "textures/UV_Distortion_Fire_NoiseTexture_Demo.png",
                    |s: &mut ImageLoaderSettings| {
                        s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..default()
                        });
                    },
                )),
            },
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}
