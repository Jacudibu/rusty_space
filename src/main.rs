use crate::asteroid_system::AsteroidPlugin;
use crate::camera::CameraControllerPlugin;
use crate::diagnostics::DiagnosticsPlugin;
use crate::entity_selection::EntitySelectionPlugin;
use crate::game_data::GameData;
use crate::gizmos::GizmoPlugin;
use crate::production::ProductionPlugin;
use crate::session_data::SessionData;
use crate::ship_ai::ShipAiPlugin;
use crate::test_universe::TestUniversePlugin;
use crate::utils::SimulationTimePlugin;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::prelude::{
    App, AppExtStates, Camera2dBundle, Commands, Handle, Image, ImagePlugin, IntoSystemConfigs,
    PluginGroup, PreUpdate, Res, Resource, Startup, Update, Window, WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
use bevy_egui::{EguiPlugin, EguiStartupSet};

mod asteroid_system;
mod camera;
mod components;
mod constants;
mod diagnostics;
mod entity_selection;
mod game_data;
mod gizmos;
mod gui;
mod map_layout;
mod physics;
mod production;
mod session_data;
mod ship_ai;
mod test_universe;
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
    .add_plugins(SimulationTimePlugin)
    .add_plugins(GizmoPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(CameraControllerPlugin)
    .add_plugins(TestUniversePlugin)
    .add_plugins(DiagnosticsPlugin)
    .add_plugins(physics::PhysicsPlugin)
    .insert_resource(GameData::mock_data())
    .insert_resource(SessionData::mock_data())
    .init_state::<gui::MouseCursorOverUiState>()
    .add_systems(
        Startup,
        (
            initialize_data,
            gui::initialize
                .after(EguiStartupSet::InitContexts)
                .after(initialize_data),
        ),
    )
    .add_systems(PreUpdate, gui::detect_mouse_cursor_over_ui)
    .add_systems(
        Update,
        (
            gui::draw_sector_info,
            gui::list_selection_icons_and_counts,
            gui::list_selection_details,
        ),
    );

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
pub struct SpriteHandles {
    asteroid: Handle<Image>,
    asteroid_selected: Handle<Image>,
    gate: Handle<Image>,
    gate_selected: Handle<Image>,
    ship: Handle<Image>,
    ship_selected: Handle<Image>,
    station: Handle<Image>,
    station_selected: Handle<Image>,
}

pub fn initialize_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprites = SpriteHandles {
        asteroid: asset_server.load("asteroid.png"),
        asteroid_selected: asset_server.load("asteroid_selected.png"),
        gate: asset_server.load("gate.png"),
        gate_selected: asset_server.load("gate_selected.png"),
        ship: asset_server.load("ship.png"),
        ship_selected: asset_server.load("ship_selected.png"),
        station: asset_server.load("station.png"),
        station_selected: asset_server.load("station_selected.png"),
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
