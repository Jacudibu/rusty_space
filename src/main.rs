use bevy::DefaultPlugins;
use bevy::asset::AssetServer;
use bevy::prelude::{
    App, Assets, Commands, Image, ImagePlugin, PluginGroup, Res, ResMut, Startup, Window,
    WindowPlugin,
};
use common::game_data::{GameData, image_generator};
use common::types::sprite_handles::SpriteHandles;
use common::{constants, session_data};

mod construction_site_placement;
mod diagnostics;
mod gizmos;
mod gui;
mod test_universe;

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
        bevy_egui::EguiPlugin::default(),
        camera::CameraPlugin,
        construction_site_placement::ConstructionSitePlacementPlugin,
        common::CommonPlugin,
        diagnostics::DiagnosticsPlugin,
        entity_selection::plugin::EntitySelectionPlugin,
        gizmos::GizmoPlugin,
        gui::GUIPlugin,
        session_data::SessionDataPlugin,
        ship_ai::ShipAiPlugin,
        ship_user_controller::ShipControllerPlugin,
        simulation::plugin::SimulationPlugin,
        test_universe::TestUniverseDataPlugin,
        universe_loader::UniverseLoadingPlugin,
        entity_spawners::plugin,
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
