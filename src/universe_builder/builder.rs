use crate::universe_builder::{gate_builder, sector_builder, ship_builder, station_builder};

#[derive(Default)]
pub struct UniverseBuilder {
    pub sectors: sector_builder::SectorSpawnData,
    pub gates: gate_builder::GateSpawnData,
    pub ships: ship_builder::ShipSpawnData,
    pub stations: station_builder::StationSpawnData,
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::asteroids::SectorWasSpawnedEvent;
    use crate::game_data::GameData;
    use crate::map_layout::MapLayout;
    use crate::universe_builder::plugin::UniverseBuilderPlugin;
    use crate::SpriteHandles;
    use bevy::prelude::*;

    impl UniverseBuilder {
        pub fn build_test_app(self) -> App {
            let mut app = App::new();
            app.init_resource::<MapLayout>();
            app.init_resource::<SpriteHandles>();
            app.add_event::<SectorWasSpawnedEvent>();
            app.insert_resource(self.sectors);
            app.insert_resource(self.gates);
            app.insert_resource(self.ships);
            app.insert_resource(self.stations);
            app.insert_resource(GameData::mock_data());

            app.add_plugins(UniverseBuilderPlugin);
            app.finish();
            app.update();

            app
        }
    }
}
