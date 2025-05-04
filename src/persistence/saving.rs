use crate::persistence::AllEntityIdMaps;
use crate::persistence::data::v1::*;
use crate::persistence::writer::sector_writer::SectorSaveDataQuery;
use crate::simulation::physics::{ConstantVelocity, ShipVelocity};
use crate::simulation::production::{ProductionFacility, Shipyard};
use crate::simulation::ship_ai::{AutoMineBehavior, AutoTradeBehavior, TaskQueue};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use bevy::prelude::{Commands, Name, Query};
use common::components::celestials::Star;
use common::components::{
    Asteroid, BuyOrders, Gate, InSector, Inventory, Sector, SellOrders, Ship, Station,
};

/// Stores all relevant entities in SaveDataCollection Resources.
/// Ideally, later on this should be completely decoupled from the main loop, maybe start an async
/// task running in the background after copying all relevant data to write stuff to disk and such.
///
/// For as long as there is no "public test build", save data is not guaranteed to be compatible
/// with older/newer versions of the game - implementing and maintaining versioning for rapidly
/// changing data structures is way too much work.
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)] // Haha, like, uh, yeah. No.
#[allow(unused)] // That's gonna fix itself as soon as we actually load stuff from disk
pub fn parse_session_data_into_universe_save_data(
    mut commands: Commands,
    all_sectors: Query<&Sector>,
    stars: Query<&Star>,
    asteroids: Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
    gates: Query<(&Gate, &InSector, &SimulationTransform)>,
    sectors_to_save: Query<SectorSaveDataQuery>,
    ships: Query<(
        &Ship,
        &Name,
        &InSector,
        &SimulationTransform,
        &TaskQueue,
        &ShipVelocity,
        &Inventory,
        Option<&AutoTradeBehavior>,
        Option<&AutoMineBehavior>,
    )>,
    stations: Query<(
        &Station,
        &Name,
        &InSector,
        &SimulationTransform,
        &Inventory,
        Option<&ProductionFacility>,
        Option<&Shipyard>,
        Option<&BuyOrders>,
        Option<&SellOrders>,
    )>,
    all_entity_id_maps: AllEntityIdMaps,
) {
    let gate_pairs = GatePairSaveData::extract_from_sector_query(&all_sectors, &gates);

    let stations = stations
        .iter()
        .map(|query_content| StationSaveData::from(query_content, &all_sectors));

    let ships = ships
        .iter()
        .map(|query_content| ShipSaveData::from(query_content, &all_sectors, &all_entity_id_maps));

    let sectors = sectors_to_save
        .iter()
        .map(|x| SectorSaveData::from(x, &asteroids, &stars));

    commands.insert_resource(SaveDataCollection::<SectorSaveData>::from(sectors));
    commands.insert_resource(SaveDataCollection::<GatePairSaveData>::from(gate_pairs));
    commands.insert_resource(SaveDataCollection::<ShipSaveData>::from(ships));
    commands.insert_resource(SaveDataCollection::<StationSaveData>::from(stations));
}

#[cfg(test)]
mod tests {
    // const CENTER: Hex = Hex::new(0, 0);
    // const RIGHT: Hex = Hex::new(1, 0);

    #[test]
    #[ignore = "Saving was implemented as a PoC, but is not a priority to keep working right now"]
    fn test_loading_then_saving_should_yield_equal_results() {
        // let mut loaded_data = UniverseSaveData::default();
        // loaded_data.sectors.add(CENTER);
        // loaded_data.sectors.add(RIGHT);
        // loaded_data.gate_pairs.add(
        //     LocalHexPosition::new(CENTER, Vec2::X),
        //     LocalHexPosition::new(RIGHT, Vec2::NEG_X),
        // );
        // loaded_data.ships.add(
        //     todo!(),
        //     LocalHexPosition::new(CENTER, Vec2::Y),
        //     2.0,
        //     String::from("Fancy test ship"),
        //     ShipBehaviorSaveData::AutoTrade {
        //         next_idle_update: SimulationTimestamp::from(249),
        //     },
        // );
        // loaded_data.stations.add(
        //     LocalHexPosition::new(RIGHT, Vec2::NEG_Y),
        //     String::from("Fancy test station"),
        // );
        //
        // let mut app = loaded_data.clone().build_test_app();
        // let world = app.world_mut();
        //
        // world.run_system_once(parse_session_data_into_universe_save_data);
        // let saved_data = UniverseSaveData {
        //     sectors: world
        //         .remove_resource::<SaveDataCollection<SectorSaveData>>()
        //         .unwrap(),
        //     gate_pairs: world
        //         .remove_resource::<SaveDataCollection<GatePairSaveData>>()
        //         .unwrap(),
        //     stations: world
        //         .remove_resource::<SaveDataCollection<StationSaveData>>()
        //         .unwrap(),
        //     ships: world
        //         .remove_resource::<SaveDataCollection<ShipSaveData>>()
        //         .unwrap(),
        // };
        //
        // assert_eq!(
        //     loaded_data, saved_data,
        //     "Save data wasn't equal after loading and saving. Maybe stuff isn't ordered correctly?"
        // );
    }
}
