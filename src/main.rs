use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{EulerRot, Vec3};
use bevy::prelude::{
    default, App, Camera2dBundle, Commands, Component, Entity, ImagePlugin, IntoSystemConfigs,
    PluginGroup, Quat, Query, Res, Startup, Time, Transform, Update, Visibility,
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
            Engine {
                forward_thrust: (i % 100) as f32,
                ..default()
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
    mut ships: Query<(Entity, &AI, &mut Engine)>,
    all_transforms: Query<&Transform>,
) {
    ships
        .par_iter_mut()
        .for_each(|(entity, ai, mut engine)| match ai {
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

                    if angle_difference - engine.rotational_thrust > 0.0 {
                        engine.turn_left(time.delta_seconds());
                    } else {
                        engine.turn_right(time.delta_seconds());
                    }

                    if angle_difference.abs() > std::f32::consts::FRAC_PI_3 {
                        engine.decelerate(time.delta_seconds());
                    } else if delta.length() > 10.0 {
                        engine.accelerate(time.delta_seconds());
                    } else {
                        engine.decelerate(time.delta_seconds());
                    }
                } else {
                    todo!()
                }
            }
        });
}

pub fn process_ship_movement(time: Res<Time>, mut ships: Query<(&mut Transform, &Engine)>) {
    ships.par_iter_mut().for_each(|(mut transform, engine)| {
        transform.rotate_z(engine.rotational_thrust * time.delta_seconds());

        let dir = transform.up();
        transform.translation += dir * engine.forward_thrust * time.delta_seconds();
    });
}

#[derive(Component)]
pub enum AI {
    MoveTo(Entity),
}

#[derive(Component)]
pub struct Engine {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub forward_thrust: f32,

    pub max_rotational: f32,
    pub rotational_strength: f32,
    pub rotational_thrust: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            max_speed: 100.0,
            acceleration: 10.0,
            deceleration: 10.0,
            forward_thrust: 0.0,
            max_rotational: 1.0,
            rotational_thrust: 0.0,
            rotational_strength: 1.0,
        }
    }
}

impl Engine {
    pub fn accelerate(&mut self, delta_seconds: f32) {
        self.forward_thrust += self.acceleration * delta_seconds;
        if self.forward_thrust > self.max_speed {
            self.forward_thrust = self.max_speed;
        }
    }

    pub fn decelerate(&mut self, delta_seconds: f32) {
        self.forward_thrust -= self.deceleration * delta_seconds;
        if self.forward_thrust < 0.0 {
            self.forward_thrust = 0.0;
        }
    }

    pub fn turn_right(&mut self, delta_seconds: f32) {
        self.rotational_thrust -= self.rotational_strength * delta_seconds;
        if self.rotational_thrust < -self.max_rotational {
            self.rotational_thrust = -self.max_rotational;
        }
    }

    pub fn turn_left(&mut self, delta_seconds: f32) {
        self.rotational_thrust += self.rotational_strength * delta_seconds;
        if self.rotational_thrust > self.max_rotational {
            self.rotational_thrust = self.max_rotational;
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
