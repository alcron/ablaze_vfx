use bevy::prelude::*;

#[derive(Clone)]
pub struct ParticleConfig {
    pub max_lifetime: f32,
    pub velocity: Vec3,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            max_lifetime: 1.0,
            velocity: Vec3::ZERO,
        }
    }
}

#[derive(Component, Clone)]
pub struct Particle {
    pub config: ParticleConfig,
    pub age: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            age: 0.0,
            config: ParticleConfig::default(),
        }
    }
}

impl Particle {
    pub fn is_alive(&self) -> bool {
        self.age < self.config.max_lifetime
    }

    pub fn age_ratio(&self) -> f32 {
        self.age / self.config.max_lifetime
    }
}

#[derive(Clone)]
pub enum Lifetime {
    Static(f32),
    Random { min: f32, max: f32 },
}

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Emitter {
    /// The rate at which particles are spawned (particles per second).
    pub spawn_rate: f32,
    pub spawn_timer: Timer,
    pub shape: EmissionShape,
    pub particles_lifetime: Lifetime,
}

impl Default for Emitter {
    fn default() -> Self {
        let spawn_rate = 10.0;

        Self {
            spawn_rate,
            spawn_timer: Timer::from_seconds(1.0 / spawn_rate, TimerMode::Repeating),
            shape: EmissionShape::Point,
            particles_lifetime: Lifetime::Static(1.0),
        }
    }
}

#[derive(Clone, Default)]
pub enum EmissionShape {
    #[default]
    Point,
    Circle {
        radius: f32,
    },
    Cone {
        angle: f32,
    },
}

#[derive(Component)]
pub struct EmitterDebugCircle;
