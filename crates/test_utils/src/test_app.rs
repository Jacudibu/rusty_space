use bevy::app::TaskPoolPlugin;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy::image::Image;
use bevy::prelude::{App, AppExtStates, State};
use bevy::state::app::StatesPlugin;
use common::game_data::GameData;
use common::session_data::SessionData;
use common::states::ApplicationState;
use common::types::map_layout::MapLayout;
use common::types::precomputed_orbit_directions::PrecomputedOrbitDirections;
use common::types::sprite_handles::SpriteHandles;
use universe_builder::gate_builder::GatePairBuilder;
use universe_builder::sector_builder::SectorBuilder;
use universe_builder::ship_builder::ShipBuilder;
use universe_builder::station_builder::StationBuilder;
use universe_loader::UniverseLoadingPlugin;

/// Helps us to quickly build a barebones bevy [App] within tests.
#[derive(Default)]
pub struct TestApp {
    pub sectors: SectorBuilder,
    pub gate_pairs: GatePairBuilder,
    pub stations: StationBuilder,
    pub ships: ShipBuilder,
}

impl TestApp {
    pub fn with_stations() {}
    pub fn with_sectors() {}
    pub fn with_ships() {}
    pub fn with_gate_pairs() {}

    /// Transforms the TestApp into a Bevy app, including the minimal set of resources and plugins necessary to run logic.
    pub fn build(self) -> App {
        let mut app = App::new();

        app.init_resource::<MapLayout>();
        app.insert_resource(create_empty_sprite_handles());
        app.init_resource::<PrecomputedOrbitDirections>();

        // all of these are required to get GameData::from_world working
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(StatesPlugin);
        app.add_plugins(AssetPlugin {
            // file_path: "../assets/".into(),
            ..Default::default()
        });
        app.init_asset::<Image>();

        GameData::initialize_mock_data(app.world_mut());
        SessionData::initialize_mock_data(app.world_mut());

        app.insert_resource(self.sectors.build());
        app.insert_resource(self.gate_pairs.build());
        app.insert_resource(self.stations.build());
        app.insert_resource(self.ships.build());

        app.add_plugins(UniverseLoadingPlugin);
        app.insert_state(ApplicationState::LoadingUniverse);

        app.finish();

        // Loading is separated into multiple steps, this way we make sure all steps are executed before we proceed.
        while still_loading(&app) {
            app.update();
        }
        app
    }
}

fn still_loading(app: &App) -> bool {
    app.world()
        .get_resource::<State<ApplicationState>>()
        .unwrap()
        .get()
        == &ApplicationState::LoadingUniverse
}

fn create_empty_sprite_handles() -> SpriteHandles {
    SpriteHandles {
        gate: Default::default(),
        gate_selected: Default::default(),
        planet: Default::default(),
        planet_selected: Default::default(),
        star: Default::default(),
        star_selected: Default::default(),
        station: Default::default(),
        station_selected: Default::default(),
        construction_site: Default::default(),
        icon_unknown: Default::default(),
        icon_ship: Default::default(),
    }
}
