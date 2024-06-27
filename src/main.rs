use crate::data::GameData;
use crate::entity_selection::MouseInteractionGizmos;
use crate::mouse_cursor::MouseCursor;
use crate::production::ProductionPlugin;
use crate::simulation_time::SimulationTime;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{
    App, AppExtStates, AppGizmoBuilder, Camera2dBundle, Commands, First, Handle, Image,
    ImagePlugin, IntoSystemConfigs, PluginGroup, PreUpdate, Res, Resource, Startup, Update, Window,
    WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

mod camera;
mod components;
mod constants;
mod data;
mod entity_selection;
mod gui;
mod mock_helpers;
mod mouse_cursor;
mod physics;
mod production;
mod ship_ai;
mod simulation_time;
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
    .insert_resource(GameData::mock_data())
    .insert_resource(MouseCursor::default())
    .insert_resource(SimulationTime::default())
    .init_gizmo_group::<MouseInteractionGizmos>()
    .init_state::<gui::MouseCursorOverUiState>()
    .add_event::<ship_ai::TaskFinishedEvent>()
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
    .add_systems(
        PreUpdate,
        (
            entity_selection::update_cursor_position,
            gui::detect_mouse_cursor_over_ui,
        ),
    )
    .add_systems(
        Update,
        (
            gui::list_selection_icons_and_counts,
            gui::list_selection_details,
            camera::move_camera,
            camera::zoom_camera,
            entity_selection::process_mouse_clicks,
            entity_selection::update_mouse_interaction,
            entity_selection::draw_mouse_interactions,
            entity_selection::on_selection_changed
                .after(entity_selection::process_mouse_clicks)
                .after(entity_selection::update_mouse_interaction),
            ship_ai::handle_idle_ships,
            ship_ai::run_ship_tasks,
            ship_ai::complete_tasks.after(ship_ai::run_ship_tasks),
            physics::move_things.after(ship_ai::run_ship_tasks),
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
