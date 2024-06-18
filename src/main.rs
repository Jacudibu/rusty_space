use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{
    default, App, Camera2dBundle, Commands, ImagePlugin, IntoSystemConfigs, PluginGroup, Quat, Res,
    Startup, Transform, Update,
};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::DefaultPlugins;
use components::*;

mod camera;
mod components;
mod physics;
mod ship_ai;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_event::<ship_ai::TaskFinishedEvent>()
        .add_systems(Startup, on_startup)
        .add_systems(Update, (camera::move_camera, camera::zoom_camera))
        .add_systems(
            Update,
            (
                ship_ai::run_ship_tasks,
                physics::move_things.after(ship_ai::run_ship_tasks),
                ship_ai::complete_tasks.after(ship_ai::run_ship_tasks),
                ship_ai::handle_idle_ships,
            ),
        )
        .run();
}

pub fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((Name::new("Camera"), camera::MainCamera, camera_bundle));

    commands.spawn((
        Name::new("Station A"),
        SpriteBundle {
            texture: asset_server.load("station.png"),
            transform: Transform::from_xyz(-200.0, -200.0, 0.0),
            ..default()
        },
        Storage::new(100.0),
        TradeHub {
            buying: true,
            selling: false,
        },
    ));

    commands.spawn((
        Name::new("Station B"),
        SpriteBundle {
            texture: asset_server.load("station.png"),
            transform: Transform::from_xyz(200.0, 200.0, 0.0),
            ..default()
        },
        Storage::new(100.0),
        TradeHub {
            buying: false,
            selling: true,
        },
    ));

    let ship_count = 1000000;
    for i in 0..ship_count {
        commands.spawn((
            Name::new("Ship"),
            ShipBehavior::AutoTrade(AutoTradeData {}),
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
