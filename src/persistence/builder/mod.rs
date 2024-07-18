//! This Module provides builder methods for the latest persistent data version.
//! Can be used for hard-coded maps during debugging and tutorials or tests.

use bevy::app::{App, Plugin};
use bevy::prelude::IntoSystemConfigs;

mod gate;
mod plugin;
mod sector;
mod ship;
mod station;

pub use plugin::UniverseSaveDataLoadingPlugin;

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::asteroids::SectorWasSpawnedEvent;
    use crate::game_data::GameData;
    use crate::map_layout::MapLayout;
    use crate::persistence::data::v1::UniverseSaveData;
    use crate::SpriteHandles;
    use bevy::prelude::*;

    impl UniverseSaveData {
        pub fn build_test_app(self) -> App {
            let mut app = App::new();
            app.init_resource::<MapLayout>();
            app.init_resource::<SpriteHandles>();
            app.add_event::<SectorWasSpawnedEvent>();
            app.insert_resource(GameData::mock_data());
            app.finish();
            app.update();

            app
        }
    }
}
