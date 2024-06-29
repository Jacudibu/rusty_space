use crate::entity_selection::EntitySelectionPlugin;
use crate::game_data::GameData;
use crate::production::ProductionPlugin;
use crate::session_data::SessionData;
use crate::ship_ai::ShipAiPlugin;
use crate::simulation_time::SimulationTime;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{
    App, AppExtStates, Camera2dBundle, Commands, First, Handle, Image, ImagePlugin,
    IntoSystemConfigs, PluginGroup, PreUpdate, Res, Resource, Startup, Update, Window,
    WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

mod camera;
mod components;
mod constants;
mod entity_selection;
mod game_data;
mod gui;
mod mock_helpers;
mod physics;
mod production;
mod session_data;
mod ship_ai;
mod simulation_time;
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
    .add_plugins(EguiPlugin)
    .add_plugins(ProductionPlugin)
    .add_plugins(EntitySelectionPlugin)
    .add_plugins(ShipAiPlugin)
    .insert_resource(GameData::mock_data())
    .insert_resource(SessionData::mock_data())
    .insert_resource(SimulationTime::default())
    .init_state::<gui::MouseCursorOverUiState>()
    .add_systems(
        Startup,
        (
            initialize_data,
            (
                mock_helpers::spawn_mock_ships,
                mock_helpers::spawn_mock_stations,
                gui::initialize,
            )
                .after(initialize_data),
        ),
    )
    .add_systems(First, simulation_time::update.after(bevy::time::TimeSystem))
    .add_systems(PreUpdate, gui::detect_mouse_cursor_over_ui)
    .add_systems(
        Update,
        (
            gui::list_selection_icons_and_counts,
            gui::list_selection_details,
            camera::move_camera,
            camera::zoom_camera,
            physics::move_things.after(ship_ai::MoveToEntity::run_tasks),
        ),
    );

    if constants::SHIP_COUNT > 10000 {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}

fn get_window_title() -> String {
    let config = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };

    format!("{} ships [{config}]", constants::SHIP_COUNT)
}

#[derive(Resource)]
pub struct SpriteHandles {
    station: Handle<Image>,
    station_selected: Handle<Image>,
    ship: Handle<Image>,
    ship_selected: Handle<Image>,
}

pub fn initialize_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprites = SpriteHandles {
        station: asset_server.load("station.png"),
        station_selected: asset_server.load("station_selected.png"),
        ship: asset_server.load("ship.png"),
        ship_selected: asset_server.load("ship_selected.png"),
    };
    commands.insert_resource(sprites);

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((Name::new("Camera"), camera::MainCamera, camera_bundle));
}
