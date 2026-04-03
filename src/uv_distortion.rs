use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    ui_widgets::ValueChange,
};

pub struct UVDistortionPlugin;

impl Plugin for UVDistortionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    noise_texture: Option<Handle<Image>>,
    #[uniform(4)]
    center_color: LinearRgba,
    #[uniform(5)]
    edge_color: LinearRgba,
    #[uniform(6)]
    color_intensity: f32,

    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/uv_distortion.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let center_color = Srgba::hex("#FFFF00FF").unwrap();
    let edge_color = Color::from(Hsla::from(center_color).with_hue(10.0));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0).subdivisions(8))),
        MeshMaterial3d(materials.add(CustomMaterial {
            texture: Some(asset_server.load("textures/UVDistortion_Fire_Demo.png")),
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
            center_color: center_color.into(),
            edge_color: edge_color.into(),
            color_intensity: 20.0,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}
