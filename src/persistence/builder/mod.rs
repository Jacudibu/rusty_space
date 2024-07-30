//! This Module provides builder methods for the latest persistent data version.
//! Can be used for hard-coded maps during debugging and tutorials or tests.

pub mod gate;
pub mod planet;
pub mod sector;
pub mod ship;
pub mod station;

pub use crate::persistence::loading_plugin::UniverseSaveDataLoadingOnStartupPlugin;

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::game_data::GameData;
    use crate::map_layout::MapLayout;
    use crate::persistence::data::v1::UniverseSaveData;
    use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
    use crate::SpriteHandles;
    use bevy::prelude::*;

    impl UniverseSaveData {
        pub fn build_test_app(self) -> App {
            let mut app = App::new();
            app.init_resource::<MapLayout>();
            app.init_resource::<SpriteHandles>();
            app.init_resource::<PrecomputedOrbitDirections>();
            app.insert_resource(GameData::mock_data());
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
