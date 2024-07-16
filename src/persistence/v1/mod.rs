use crate::components::{
    Asteroid, BuyOrders, Gate, InSector, Inventory, Sector, SellOrders, Ship, Station,
};
use crate::persistence::v1::gate_save_data::GatePairSaveData;
use crate::persistence::v1::sector_save_data::SectorSaveData;
use crate::persistence::AllEntityIdMaps;
use crate::physics::{ConstantVelocity, ShipVelocity};
use crate::production::{ProductionComponent, ShipyardComponent};
use crate::ship_ai::TaskQueue;
use bevy::prelude::{Name, Query, Transform};
use serde::{Deserialize, Serialize};
use ship_save_data::ShipSaveData;
use station_save_data::StationSaveData;

mod gate_save_data;
mod inventory_save_data;
mod sector_save_data;
mod ship_save_data;
mod station_save_data;
mod task_save_data;

#[derive(Serialize, Deserialize)]
pub struct UniverseSaveData {
    gate_pairs: Vec<GatePairSaveData>,
    sectors: Vec<SectorSaveData>,
    ships: Vec<ShipSaveData>,
    stations: Vec<StationSaveData>,
}

/// For as long as there is no "public test build", save data is not guaranteed to be compatible with older/newer versions of the game.
#[allow(clippy::type_complexity)] // Haha, like, uh, yeah. No.
pub fn parse_session_data_into_universe_save_data(
    asteroids: Query<(&Asteroid, &Transform, &ConstantVelocity)>,
    gates: Query<(&Gate, &InSector, &Transform)>,
    sectors: Query<&Sector>,
    ships: Query<(
        &Ship,
        &Name,
        &Transform,
        &TaskQueue,
        &ShipVelocity,
        &Inventory,
    )>,
    stations: Query<(
        &Station,
        &Name,
        &InSector,
        &Transform,
        &Inventory,
        Option<&ProductionComponent>,
        Option<&ShipyardComponent>,
        Option<&BuyOrders>,
        Option<&SellOrders>,
    )>,
    all_entity_id_maps: AllEntityIdMaps,
) -> UniverseSaveData {
    let gate_pairs =
        GatePairSaveData::extract_from_sector_query(&sectors, &gates, &all_entity_id_maps);

    let sectors = sectors
        .iter()
        .map(|x| SectorSaveData::from(x, &asteroids))
        .collect();

    let ships = ships
        .iter()
        .map(|query_content| ShipSaveData::from(query_content, &all_entity_id_maps))
        .collect();

    let stations = stations
        .iter()
        .map(|query_content| StationSaveData::from(query_content, &all_entity_id_maps))
        .collect();

    UniverseSaveData {
        gate_pairs,
        sectors,
        ships,
        stations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universe_builder::{LocalHexPosition, UniverseBuilder};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Vec2;
    use hexx::Hex;

    const CENTER: Hex = Hex::new(0, 0);
    const RIGHT: Hex = Hex::new(1, 0);

    #[test]
    fn test_serializing() {
        let mut universe_builder = UniverseBuilder::default();
        universe_builder.sectors.add(CENTER);
        universe_builder.sectors.add(RIGHT);
        universe_builder.gates.add(
            LocalHexPosition::new(CENTER, Vec2::ZERO),
            LocalHexPosition::new(RIGHT, Vec2::ZERO),
        );

        let mut app = universe_builder.build_test_app();
        let world = app.world_mut();

        world.run_system_once(parse_session_data_into_universe_save_data);
    }
}
