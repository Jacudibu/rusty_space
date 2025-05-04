use crate::game_data::GameData;
use bevy::DefaultPlugins;
use bevy::asset::AssetServer;
use bevy::prelude::{
    App, Assets, Commands, Handle, Image, ImagePlugin, PluginGroup, Res, ResMut, Resource, Startup,
    Window, WindowPlugin,
};

mod components;
mod constants;
mod construction_site_placement;
mod diagnostics;
mod entity_selection;
mod game_data;
mod gizmos;
mod gui;
mod image_generator;
mod map_layout;
mod pathfinding;
mod persistence;
mod session_data;
mod simulation;
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
    );

    GameData::initialize_mock_data(app.world_mut());

    app.add_plugins((
        bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        camera::CameraPlugin,
        common::CommonPlugin,
        // diagnostics::DiagnosticsPlugin,
        entity_selection::EntitySelectionPlugin,
        gizmos::GizmoPlugin,
        gui::GUIPlugin,
        persistence::UniverseSaveDataLoadingOnStartupPlugin,
        persistence::test_universe::TestUniverseDataPlugin,
        simulation::plugin::SimulationPlugin,
        session_data::SessionDataPlugin,
        construction_site_placement::ConstructionSitePlacementPlugin,
    ))
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
        constants::TRADE_SHIP_COUNT
            + constants::MINING_SHIP_COUNT
            + constants::HARVESTING_SHIP_COUNT
    )
}

#[derive(Resource)]
#[cfg_attr(test, derive(Default))]
pub struct SpriteHandles {
    gate: Handle<Image>,
    gate_selected: Handle<Image>,
    planet: Handle<Image>,
    planet_selected: Handle<Image>,
    star: Handle<Image>,
    star_selected: Handle<Image>,
    station: Handle<Image>,
    station_selected: Handle<Image>,
    construction_site: Handle<Image>,
    icon_unknown: Handle<Image>,
    icon_ship: Handle<Image>,
}

pub fn initialize_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    let sprites = SpriteHandles {
        gate: asset_server.load("sprites/gate.png"),
        gate_selected: image_generator::generate_image_with_highlighted_corners_from_asset_path(
            "sprites/gate.png",
            &mut image_assets,
        ),
        planet: asset_server.load("sprites/planet.png"),
        planet_selected: image_generator::generate_image_with_highlighted_corners_from_asset_path(
            "sprites/planet.png",
            &mut image_assets,
        ),
        star: asset_server.load("sprites/star.png"),
        star_selected: image_generator::generate_image_with_highlighted_corners_from_asset_path(
            "sprites/star.png",
            &mut image_assets,
        ),
        station: asset_server.load("sprites/station.png"),
        station_selected: image_generator::generate_image_with_highlighted_corners_from_asset_path(
            "sprites/station.png",
            &mut image_assets,
        ),
        construction_site: asset_server.load("sprites/construction_site.png"),
        icon_unknown: asset_server.load("sprites/items/unknown.png"),
        icon_ship: asset_server.load("sprites/ships/ship_fighter.png"),
    };
    commands.insert_resource(sprites);
}
