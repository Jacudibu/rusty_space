//! This Module provides builder methods for the latest persistent data version.
//! Can be used for hard-coded maps during debugging and tutorials or tests.

pub mod celestial_builder;
pub mod gate_builder;
pub mod sector_builder;
pub mod ship_builder;
pub mod station_builder;

pub use crate::persistence::loading_plugin::UniverseSaveDataLoadingOnStartupPlugin;

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::SpriteHandles;
    use crate::map_layout::MapLayout;
    use crate::persistence::data::v1::UniverseSaveData;
    use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
    use bevy::prelude::*;
    use common::game_data::GameData;
    use common::session_data::SessionData;

    impl UniverseSaveData {
        pub fn build_test_app(self) -> App {
            let mut app = App::new();
            app.init_resource::<MapLayout>();
            app.init_resource::<SpriteHandles>();
            app.init_resource::<PrecomputedOrbitDirections>();

            // all of these are required to get GameData::from_world working
            app.add_plugins(TaskPoolPlugin::default());
            app.add_plugins(AssetPlugin::default());
            app.init_asset::<Image>();

            GameData::initialize_mock_data(app.world_mut());
            SessionData::initialize_mock_data(app.world_mut());

            app.insert_resource(self.sectors);
            app.insert_resource(self.gate_pairs);
            app.insert_resource(self.stations);
            app.insert_resource(self.ships);

            app.add_plugins(UniverseSaveDataLoadingOnStartupPlugin);
            app.finish();
            app.update();

            app
        }
    }
}
