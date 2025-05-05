use bevy::app::TaskPoolPlugin;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy::image::Image;
use bevy::prelude::App;
use common::game_data::GameData;
use common::session_data::SessionData;
use common::types::map_layout::MapLayout;
use common::types::precomputed_orbit_directions::PrecomputedOrbitDirections;
use common::types::sprite_handles::SpriteHandles;
use persistence::data::UniverseSaveData;

/// Helps us to quickly build a barebones bevy [App] within tests.
pub struct TestApp {
    pub data: UniverseSaveData,
}

impl TestApp {
    pub fn with_stations() {}
    pub fn with_sectors() {}
    pub fn with_ships() {}
    pub fn with_gate_pairs() {}

    pub fn build(self) -> App {
        let mut app = App::new();

        app.init_resource::<MapLayout>();
        app.insert_resource(create_empty_sprite_handles());
        app.init_resource::<PrecomputedOrbitDirections>();

        // all of these are required to get GameData::from_world working
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Image>();

        GameData::initialize_mock_data(app.world_mut());
        SessionData::initialize_mock_data(app.world_mut());

        // TODO
        // app.insert_resource(self.data.sectors);
        // app.insert_resource(self.data.gate_pairs);
        // app.insert_resource(self.data.stations);
        // app.insert_resource(self.data.ships);

        // app.add_plugins(UniverseSaveDataLoadingOnStartupPlugin);

        app.finish();
        app.update();
        app
    }
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
