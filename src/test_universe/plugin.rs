use crate::components::SectorEntity;
use crate::initialize_data;
use crate::test_universe::spawn_test_gates::spawn_test_gates;
use crate::test_universe::spawn_test_sectors::spawn_test_sectors;
use crate::test_universe::spawn_test_ships::spawn_test_ships;
use crate::test_universe::spawn_test_stations::spawn_test_stations;
use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Resource, Startup};

/// Spawns a lot of hardcoded test stuff.
pub struct TestUniversePlugin;
impl Plugin for TestUniversePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_test_sectors,
                spawn_test_gates,
                spawn_test_stations,
                spawn_test_ships,
            )
                .after(initialize_data)
                .chain(),
        );
    }
}

#[derive(Resource)]
pub struct TestSectors {
    pub center: SectorEntity,
    pub right: SectorEntity,
    pub top_right: SectorEntity,
    pub bottom_left: SectorEntity,
}
