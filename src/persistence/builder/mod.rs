//! This Module provides builder methods for the latest persistent data version.
//! Can be used for hard-coded maps during debugging and tutorials or tests.

use crate::asteroids::SectorWasSpawnedEvent;
use crate::map_layout::MapLayout;
use crate::persistence::data::v1::UniverseSaveData;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{Commands, EventWriter, Res, World};

mod gate;
mod sector;
mod ship;
mod station;

impl UniverseSaveData {
    pub fn load(self, world: &mut World) {
        // world.run_system_once(
        //     |commands: Commands,
        //      map_layout: Res<MapLayout>,
        //      sector_spawn_event: EventWriter<SectorWasSpawnedEvent>| {
        //         self.sectors
        //             .spawn_all(commands, map_layout, sector_spawn_event)
        //     },
        // );

        // TODO: Asteroids
        // world.run_system_once(self.gate_pairs.spawn_all);
        // world.run_system_once(self.stations.spawn_all);
        // world.run_system_once(self.ships.spawn_all);
    }
}

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
