// mod particles;
// mod uv_distortion;
// mod fresnel;
// mod volumetric_fog;
// mod polar_coordinates;
mod water;

// use crate::particles::ParticlesPlugin;
// use crate::uv_distortion::UVDistortionPlugin;
// use crate::fresnel::FresnelPlugin;
// use crate::volumetric_fog::VolumetricFogPlugin;
// use crate::polar_coordinates::PolarCoordinatesPlugin;
use crate::water::WaterPlugin;

use bevy::{
    core_pipeline::prepass::DepthPrepass, post_process::bloom::Bloom, prelude::*, render::view::Hdr,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (500, 300).into(),
                    ..default()
                }),
                ..default()
            }),
            // ParticlesPlugin,
            // UVDistortionPlugin,
            // FresnelPlugin,
            // VolumetricFogPlugin,
            // PolarCoordinatesPlugin,
            WaterPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        Hdr::default(),
        Bloom::default(),
        Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // UI text
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_child((
            Text::new("Water"),
            TextFont {
                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
        ));
}

fn close_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
