pub mod components;

use self::components::{Emitter, Particle};
use crate::particles::components::{EmissionShape, Lifetime, ParticleConfig};
use bevy::{pbr::Material, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};
use rand::RngExt;
use std::f32::consts::TAU;

// TODO Export material to separate "material" file like in impatient guide.
#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,
    #[uniform(2)]
    color: LinearRgba,
    #[uniform(3)]
    age_ratio: f32,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (update_emitters, update_particles).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Emitter {
            shape: EmissionShape::Circle { radius: 0.2 },
            particles_lifetime: Lifetime::Random { min: 3.0, max: 5.0 },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
    mut emitters: Query<(Entity, &mut Emitter)>,
) {
    let mut rng = rand::rng();
    for (entity, mut emitter) in emitters.iter_mut() {
        emitter.spawn_timer.tick(time.delta());

        if emitter.spawn_timer.just_finished() {
            let particle_offset = match emitter.shape {
                EmissionShape::Point => Vec3::ZERO,
                EmissionShape::Circle { radius } => {
                    let angle = rng.random_range(0.0..TAU);
                    let distance = rng.random_range(0.0..radius);

                    Vec3::new(angle.cos() * distance, 0.0, angle.sin() * distance)
                }
                EmissionShape::Cone { angle } => {
                    Vec3::ZERO // TODO Implement cone emission shape.
                }
            };
            commands.spawn((
                Particle {
                    config: ParticleConfig {
                        max_lifetime: match emitter.particles_lifetime {
                            Lifetime::Static(lifetime) => lifetime,
                            Lifetime::Random { min, max } => rng.random_range(min..max),
                        },
                        velocity: Vec3::new(0.0, 1.0, 0.0),
                    },
                    ..default()
                },
                Mesh3d(meshes.add(Plane3d::default().mesh().size(0.7, 0.7))),
                MeshMaterial3d(materials.add(CustomMaterial {
                    texture: Some(asset_server.load("textures/GlowingDot.png")),
                    // color: Srgba::hex("#82FF00FF").unwrap().into(),
                    color: Color::hsv(80.0, 1.0, 5.0).into(),
                    age_ratio: 0.0,
                    alpha_mode: AlphaMode::Blend,
                })),
                Transform::from_translation(particle_offset),
                ChildOf(entity),
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle, &MeshMaterial3d<CustomMaterial>)>,
) {
    for (entity, mut particle, material_handle) in particles.iter_mut() {
        particle.age += time.delta_secs();

        if !particle.is_alive() {
            commands.entity(entity).despawn();
            continue;
        }

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.age_ratio = particle.age_ratio();
        }
    }
}

// fn generate_particle(config: ParticleConfig) -> impl Bundle {
//     // let offset =
// }
