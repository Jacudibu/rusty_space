use crate::initialize_data;
use crate::universe_builder::{gate_builder, sector_builder, ship_builder, station_builder};
use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, Plugin, Startup};

/// This will build a new universe, defined by the following resources which must be added before any internal systems are run:
///
/// - [sector_builder::SectorSpawnData] to define sectors
/// - [gate_builder::GateSpawnData] to define gates
/// - [station_builder::StationSpawnData] to define stations
/// - [ship_builder::ShipSpawnData] to define ships
///
/// Once spawned, these resources will be removed.
pub struct UniverseBuilderPlugin;
impl Plugin for UniverseBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sector_builder::spawn_all_sectors,
                gate_builder::spawn_all_gates,
                station_builder::spawn_all_stations,
                ship_builder::spawn_all_ships,
            )
                .after(initialize_data)
                .chain(),
        );
    }
}
