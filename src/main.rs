use crate::game_data::GameData;
use crate::session_data::SessionData;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::prelude::{
    App, Camera2dBundle, Commands, Handle, Image, ImagePlugin, PluginGroup, Res, Resource, Startup,
    Window, WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
mod camera;
mod components;
mod constants;
mod diagnostics;
mod entity_selection;
mod game_data;
mod gizmos;
mod gui;
mod map_layout;
mod pathfinding;
mod persistence;
mod session_data;
mod simulation;
mod states;
mod trade_plan;
mod utils;

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
    .add_plugins((
        bevy_egui::EguiPlugin,
        camera::CameraControllerPlugin,
        diagnostics::DiagnosticsPlugin,
        entity_selection::EntitySelectionPlugin,
        gizmos::GizmoPlugin,
        gui::GUIPlugin,
        persistence::UniverseSaveDataLoadingOnStartupPlugin,
        persistence::test_universe::TestUniverseDataPlugin,
        simulation::plugin::SimulationPlugin,
        states::StatePlugin,
    ))
    .insert_resource(GameData::mock_data())
    .insert_resource(SessionData::mock_data())
    .add_systems(Startup, initialize_data);

    app.run();
}

fn get_window_title() -> String {
    let config = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };

    format!(
        "{} ships [{config}]",
        constants::TRADE_SHIP_COUNT + constants::MINING_SHIP_COUNT
    )
}

#[derive(Resource)]
#[cfg_attr(test, derive(Default))]
pub struct SpriteHandles {
    asteroid: Handle<Image>,
    asteroid_selected: Handle<Image>,
    gate: Handle<Image>,
    gate_selected: Handle<Image>,
    planet: Handle<Image>,
    planet_selected: Handle<Image>,
    ship: Handle<Image>,
    ship_selected: Handle<Image>,
    station: Handle<Image>,
    station_selected: Handle<Image>,
    icon_item_a: Handle<Image>,
    icon_item_b: Handle<Image>,
    icon_item_c: Handle<Image>,
    icon_unknown: Handle<Image>,
    icon_ship: Handle<Image>,
}

pub fn initialize_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprites = SpriteHandles {
        asteroid: asset_server.load("asteroid.png"),
        asteroid_selected: asset_server.load("asteroid_selected.png"),
        gate: asset_server.load("gate.png"),
        gate_selected: asset_server.load("gate_selected.png"),
        planet: asset_server.load("planet.png"),
        planet_selected: asset_server.load("planet_selected.png"),
        ship: asset_server.load("ship.png"),
        ship_selected: asset_server.load("ship_selected.png"),
        station: asset_server.load("station.png"),
        station_selected: asset_server.load("station_selected.png"),
        icon_item_a: asset_server.load("ui_icons/items/a.png"),
        icon_item_b: asset_server.load("ui_icons/items/b.png"),
        icon_item_c: asset_server.load("ui_icons/items/c.png"),
        icon_unknown: asset_server.load("ui_icons/items/unknown.png"),
        icon_ship: asset_server.load("ui_icons/items/ship.png"),
    };
    commands.insert_resource(sprites);

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((
        Name::new("Camera"),
        camera::MainCameraBundle::default(),
        camera_bundle,
    ));
}
