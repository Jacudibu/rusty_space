use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec3;
use bevy::prelude::{
    default, App, Camera2dBundle, Commands, ImagePlugin, IntoSystemConfigs, PluginGroup, Quat, Res,
    Startup, Transform, Update, Window, WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::DefaultPlugins;
use components::*;

mod camera;
mod components;
mod physics;
mod ship_ai;

const SHIP_COUNT: i32 = 1000000;

fn get_window_title() -> String {
    let config = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };

    format!("{SHIP_COUNT} ships [{config}] ")
}

const SHIP_LAYER: f32 = 10.0;
const STATION_LAYER: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: get_window_title(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
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
            transform: Transform::from_xyz(-200.0, -200.0, STATION_LAYER),
            ..default()
        },
        Storage::new(100000000),
        TradeHub {
            buying: true,
            selling: false,
        },
    ));

    commands.spawn((
        Name::new("Station B"),
        SpriteBundle {
            texture: asset_server.load("station.png"),
            transform: Transform::from_xyz(200.0, 200.0, STATION_LAYER),
            ..default()
        },
        Storage {
            capacity: 100000000,
            used: 100000000,
        },
        TradeHub {
            buying: false,
            selling: true,
        },
    ));

    for i in 0..SHIP_COUNT {
        commands.spawn((
            Name::new("Ship"),
            ShipBehavior::AutoTrade(AutoTradeData {}),
            Engine { ..default() },
            Velocity {
                forward: (i % 100) as f32,
                angular: 0.0,
            },
            Storage::new(100),
            SpriteBundle {
                texture: asset_server.load("ship.png"),
                transform: Transform {
                    rotation: Quat::from_rotation_z(
                        (std::f32::consts::PI * 2.0 / SHIP_COUNT as f32) * i as f32,
                    ),
                    translation: Vec3::new(0.0, 0.0, SHIP_LAYER),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
