use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::EulerRot;
use bevy::prelude::{
    default, App, Camera2dBundle, Commands, Component, Entity, ImagePlugin, IntoSystemConfigs,
    PluginGroup, Quat, Query, Res, Startup, Time, Transform, Update,
};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, on_startup)
        .add_systems(
            Update,
            (run_ship_ai, process_ship_movement.after(run_ship_ai)),
        )
        .run();
}

pub fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((Name::new("Camera"), camera_bundle));

    let station_a = commands
        .spawn((
            Name::new("Station A"),
            SpriteBundle {
                texture: asset_server.load("station.png"),
                transform: Transform::from_xyz(-200.0, -200.0, 0.0),
                ..default()
            },
            Storage::new(100.0),
        ))
        .id();

    let ship_count = 100000;
    for i in 0..ship_count {
        commands.spawn((
            Name::new("Ship"),
            AI::MoveTo(station_a),
            Engine { ..default() },
            Velocity {
                forward: (i % 100) as f32,
                angular: 0.0,
            },
            Storage::new(100.0),
            SpriteBundle {
                texture: asset_server.load("ship.png"),
                transform: Transform {
                    rotation: Quat::from_rotation_z(
                        (std::f32::consts::PI * 2.0 / ship_count as f32) * i as f32,
                    ),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn run_ship_ai(
    time: Res<Time>,
    mut ships: Query<(Entity, &AI, &Engine, &mut Velocity)>,
    all_transforms: Query<&Transform>,
) {
    ships
        .par_iter_mut()
        .for_each(|(entity, ai, engine, mut velocity)| match ai {
            AI::MoveTo(target) => {
                if let Ok(target_transform) = all_transforms.get(*target) {
                    let entity_transform = all_transforms.get(entity).unwrap();
                    let delta = target_transform.translation.truncate()
                        - entity_transform.translation.truncate();

                    let (_, _, own_rotation) = entity_transform.rotation.to_euler(EulerRot::XYZ);
                    let own_rotation = own_rotation + std::f32::consts::FRAC_PI_2;

                    let target = delta.y.atan2(delta.x);
                    let mut angle_difference = target - own_rotation;

                    if angle_difference > std::f32::consts::PI {
                        angle_difference -= 2.0 * std::f32::consts::PI;
                    } else if angle_difference < -std::f32::consts::PI {
                        angle_difference += 2.0 * std::f32::consts::PI;
                    }

                    if angle_difference - velocity.angular > 0.0 {
                        velocity.turn_left(engine, time.delta_seconds());
                    } else {
                        velocity.turn_right(engine, time.delta_seconds());
                    }

                    if angle_difference.abs() > std::f32::consts::FRAC_PI_3 {
                        velocity.decelerate(engine, time.delta_seconds());
                    } else if delta.length() > 10.0 {
                        velocity.accelerate(engine, time.delta_seconds());
                    } else {
                        velocity.decelerate(engine, time.delta_seconds());
                    }
                } else {
                    todo!()
                }
            }
        });
}

pub fn process_ship_movement(time: Res<Time>, mut ships: Query<(&mut Transform, &Velocity)>) {
    ships.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.rotate_z(velocity.angular * time.delta_seconds());

        let forward = transform.up();
        transform.translation += forward * velocity.forward * time.delta_seconds();
    });
}

#[derive(Component)]
pub enum AI {
    MoveTo(Entity),
}

/// Fake Physics!
#[derive(Component, Default)]
pub struct Velocity {
    pub forward: f32,
    pub angular: f32,
}

impl Velocity {
    pub fn accelerate(&mut self, engine: &Engine, delta_seconds: f32) {
        self.forward += engine.acceleration * delta_seconds;
        if self.forward > engine.max_speed {
            self.forward = engine.max_speed;
        }
    }

    pub fn decelerate(&mut self, engine: &Engine, delta_seconds: f32) {
        self.forward -= engine.deceleration * delta_seconds;
        if self.forward < 0.0 {
            self.forward = 0.0;
        }
    }

    pub fn turn_right(&mut self, engine: &Engine, delta_seconds: f32) {
        self.angular -= engine.angular_acceleration * delta_seconds;
        if self.angular < -engine.max_angular_speed {
            self.angular = -engine.max_angular_speed;
        }
    }

    pub fn turn_left(&mut self, engine: &Engine, delta_seconds: f32) {
        self.angular += engine.angular_acceleration * delta_seconds;
        if self.angular > engine.max_angular_speed {
            self.angular = engine.max_angular_speed;
        }
    }
}

#[derive(Component)]
pub struct Engine {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            max_speed: 100.0,
            acceleration: 10.0,
            deceleration: 10.0,
            max_angular_speed: 1.0,
            angular_acceleration: 1.0,
        }
    }
}

#[derive(Component)]
pub struct Storage {
    pub capacity: f32,
    pub used: f32,
}

impl Storage {
    pub fn new(capacity: f32) -> Self {
        Self {
            capacity,
            used: 0.0,
        }
    }
}
