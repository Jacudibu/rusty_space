use crate::data::GameData;
use crate::entity_selection::MouseInteractionGizmos;
use crate::mouse_cursor::MouseCursor;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec3;
use bevy::prelude::{
    default, App, AppGizmoBuilder, Camera2dBundle, Commands, Handle, Image, ImagePlugin,
    IntoSystemConfigs, PluginGroup, PreUpdate, Quat, Res, Resource, Startup, Transform, Update,
    Window, WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::DefaultPlugins;
use components::*;
use data::DEBUG_ITEM_ID;

mod camera;
mod components;
mod data;
mod entity_selection;
mod mouse_cursor;
mod physics;
mod ship_ai;
mod utils;

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
    let mut app = App::new();
    app.add_plugins(
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
    .insert_resource(GameData::mock_data())
    .insert_resource(MouseCursor::default())
    .init_gizmo_group::<MouseInteractionGizmos>()
    .add_event::<ship_ai::TaskFinishedEvent>()
    .add_systems(Startup, on_startup)
    .add_systems(PreUpdate, entity_selection::update_cursor_position)
    .add_systems(
        Update,
        (
            camera::move_camera,
            camera::zoom_camera,
            entity_selection::process_mouse_clicks,
            entity_selection::update_mouse_interaction,
            entity_selection::draw_mouse_interactions,
            entity_selection::on_selection_changed.after(entity_selection::process_mouse_clicks),
            ship_ai::handle_idle_ships,
            ship_ai::run_ship_tasks,
            ship_ai::complete_tasks.after(ship_ai::run_ship_tasks),
            physics::move_things.after(ship_ai::run_ship_tasks),
        ),
    );

    if SHIP_COUNT > 10000 {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}

#[derive(Resource)]
pub struct SpriteHandles {
    station: Handle<Image>,
    station_selected: Handle<Image>,
    ship: Handle<Image>,
    ship_selected: Handle<Image>,
}

pub fn on_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_data: Res<GameData>,
) {
    let sprites = SpriteHandles {
        station: asset_server.load("station.png"),
        station_selected: asset_server.load("station_selected.png"),
        ship: asset_server.load("ship.png"),
        ship_selected: asset_server.load("ship_selected.png"),
    };

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((Name::new("Camera"), camera::MainCamera, camera_bundle));

    commands.spawn((
        Name::new("Station A"),
        SelectableEntity::Station,
        SpriteBundle {
            texture: sprites.station.clone(),
            transform: Transform::from_xyz(-200.0, -200.0, STATION_LAYER),
            ..default()
        },
        Storage::new(u32::MAX / 10),
        BuyOrders::mock_buying_item(&game_data.items[&DEBUG_ITEM_ID]),
    ));

    let mut filled_storage = Storage::new(u32::MAX / 10);
    filled_storage.add_item(DEBUG_ITEM_ID, u32::MAX / 10);

    commands.spawn((
        Name::new("Station B"),
        SelectableEntity::Station,
        SpriteBundle {
            texture: sprites.station.clone(),
            transform: Transform::from_xyz(200.0, 200.0, STATION_LAYER),
            ..default()
        },
        filled_storage,
        SellOrders::mock_selling_item(&game_data.items[&DEBUG_ITEM_ID]),
    ));

    for i in 0..SHIP_COUNT {
        commands.spawn((
            Name::new("Ship"),
            SelectableEntity::Ship,
            ShipBehavior::AutoTrade(AutoTradeData {}),
            Engine { ..default() },
            Velocity {
                forward: (i % 100) as f32,
                angular: 0.0,
            },
            Storage::new(100),
            SpriteBundle {
                texture: sprites.ship.clone(),
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

    commands.insert_resource(sprites);
}
