use crate::LoadingState;
use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, NextState, Query, Res, ResMut, State};
use common::components::celestials::Celestial;
use common::components::production_facility::{
    ProductionFacility, ProductionModule, ProductionQueueElement, RunningProductionQueueElement,
};
use common::components::ship_velocity::ShipVelocity;
use common::components::shipyard::{OngoingShipConstructionOrder, Shipyard, ShipyardModule};
use common::components::{BuyOrders, Sector, SectorWithCelestials};
use common::game_data::{
    AsteroidManifest, ItemManifest, ProductionModuleId, RecipeManifest, ShipyardModuleId,
};
use common::session_data::ShipConfigurationManifest;
use common::types::auto_mine_state::AutoMineState;
use common::types::behavior_builder::BehaviorBuilder;
use common::types::entity_id_map::{
    AsteroidIdMap, CelestialIdMap, ConstructionSiteIdMap, GateIdMap, SectorIdMap, ShipIdMap,
    StationIdMap,
};
use common::types::map_layout::MapLayout;
use common::types::sector_position::SectorPosition;
use common::types::sprite_handles::SpriteHandles;
use entity_spawners::spawn_gates::spawn_gate_pair;
use entity_spawners::spawn_sector::spawn_sector;
use entity_spawners::spawn_ship::spawn_ship;
use entity_spawners::spawn_station::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use persistence::data::{
    ActiveShipyardOrderSaveData, AutoMineStateSaveData, GatePairSaveData, ProductionModuleSaveData,
    ProductionSaveData, SaveDataCollection, SectorSaveData, ShipBehaviorSaveData, ShipSaveData,
    ShipyardModuleSaveData, ShipyardSaveData, StationSaveData,
};

#[derive(SystemParam)]
pub(crate) struct SpawnAllSectorArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    map_layout: Res<'w, MapLayout>,
    asteroid_manifest: Res<'w, AsteroidManifest>,
    sector_id_map: ResMut<'w, SectorIdMap>,
    asteroid_id_map: ResMut<'w, AsteroidIdMap>,
    planet_id_map: ResMut<'w, CelestialIdMap>,
}

pub(crate) fn spawn_all_sectors(
    mut data: ResMut<SaveDataCollection<SectorSaveData>>,
    state: Res<State<LoadingState>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllSectorArgs,
) {
    if data.data.is_empty() {
        next_state.set(state.next());
        return;
    }

    let next = data.data.pop().unwrap();

    let coordinate = next.coordinate;
    let entity = spawn_sector(
        &mut args.commands,
        &args.map_layout.hex_layout,
        next.coordinate,
        &next.features,
        &args.sprites,
        &mut args.asteroid_id_map,
        &mut args.planet_id_map,
        &args.asteroid_manifest,
    );
    args.sector_id_map.insert(coordinate, entity);
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllGatesArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorWithCelestials>)>,
    celestials: Query<'w, 's, &'static Celestial>,

    sector_id_map: ResMut<'w, SectorIdMap>,
    gate_id_map: ResMut<'w, GateIdMap>,
}

pub(crate) fn spawn_all_gates(
    mut data: ResMut<SaveDataCollection<GatePairSaveData>>,
    state: Res<State<LoadingState>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllGatesArgs,
) {
    if data.data.is_empty() {
        next_state.set(state.next());
        return;
    }

    let next = data.data.pop().unwrap();
    spawn_gate_pair(
        &mut args.commands,
        &mut args.gate_id_map,
        &mut args.sectors,
        &args.sprites,
        next.from_id,
        next.from_position.to_sector_position(&args.sector_id_map),
        next.to_id,
        next.to_position.to_sector_position(&args.sector_id_map),
    )
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllStationsArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorWithCelestials>)>,
    sector_id_map: Res<'w, SectorIdMap>,
    items: Res<'w, ItemManifest>,
    recipes: Res<'w, RecipeManifest>,
    celestials: Query<'w, 's, &'static Celestial>,

    station_id_map: ResMut<'w, StationIdMap>,
    construction_site_id_map: ResMut<'w, ConstructionSiteIdMap>,
}

pub(crate) fn spawn_all_stations(
    mut data: ResMut<SaveDataCollection<StationSaveData>>,
    state: Res<State<LoadingState>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllStationsArgs,
) {
    if data.data.is_empty() {
        next_state.set(state.next());
        return;
    }

    let next = data.data.pop().unwrap();
    let sector_entity = args
        .sector_id_map
        .get_entity(&next.position.sector)
        .unwrap();

    // TODO: All custom trade data is lost right now
    let buys = next
        .buy_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());
    let sells = next
        .sell_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());

    let production = next
        .production_modules
        .clone() // TODO: Can we get rid of those clones?
        .map(parse_production_save_data);
    let shipyard = next.shipyard_modules.clone().map(parse_shipyard_save_data);

    let next = StationSpawnData {
        id: next.id,
        name: next.name.clone(),
        sector_position: SectorPosition {
            sector: *sector_entity,
            local_position: next.position.local_position,
        },
        shipyard,
        production,
        buys,
        sells,
        construction_site: next.construction_site.clone().map(|x| {
            ConstructionSiteSpawnData::new(x.queue, BuyOrders::default()) // TODO
                .with_progress(x.current_progress)
        }),
    };

    spawn_station(
        &mut args.commands,
        &mut args.sectors,
        &mut args.station_id_map,
        &mut args.construction_site_id_map,
        &args.sprites,
        &args.items,
        &args.recipes,
        next,
    );
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllShipsArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,

    ship_id_map: ResMut<'w, ShipIdMap>,
}

pub(crate) fn spawn_all_ships(
    mut data: ResMut<SaveDataCollection<ShipSaveData>>,
    state: Res<State<LoadingState>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllShipsArgs,
) {
    if data.data.is_empty() {
        next_state.set(state.next());
        return;
    }

    /// Arbitrary number of ships that should be loaded during a single frame.
    /// As long as bigger frame rates = faster loading, this can be way higher down the line.
    const SHIPS_LOADED_AT_ONCE: usize = 432;

    let split_at = if data.data.len() > SHIPS_LOADED_AT_ONCE {
        data.data.len() - SHIPS_LOADED_AT_ONCE
    } else {
        0
    };

    let split = data.data.split_off(split_at);
    for next in split.into_iter() {
        spawn_ship(
            &mut args.commands,
            next.id,
            next.name.clone(),
            &mut args.sectors,
            args.sector_id_map.id_to_entity()[&next.position.sector],
            next.position.local_position,
            next.rotation_degrees,
            ShipVelocity {
                forward: next.forward_velocity,
                angular: next.angular_velocity,
            },
            convert_behavior_save_data_to_builder_data(next.behavior),
            &mut args.ship_id_map,
            args.ship_configurations.get_by_id(&next.config_id).unwrap(),
        );
    }
}

pub fn convert_behavior_save_data_to_builder_data(value: ShipBehaviorSaveData) -> BehaviorBuilder {
    match value {
        ShipBehaviorSaveData::AutoTrade => BehaviorBuilder::AutoTrade,
        ShipBehaviorSaveData::AutoConstruct => BehaviorBuilder::AutoConstruct,
        ShipBehaviorSaveData::AutoMine { mined_ore, state } => BehaviorBuilder::AutoMine {
            mined_ore,
            state: convert_auto_mine_state(state),
        },
        ShipBehaviorSaveData::AutoHarvest {
            harvested_gas,
            state,
        } => BehaviorBuilder::AutoHarvest {
            harvested_gas,
            state: convert_auto_mine_state(state),
        },
        ShipBehaviorSaveData::HoldPosition => BehaviorBuilder::HoldPosition,
    }
}

fn convert_auto_mine_state(state: AutoMineStateSaveData) -> AutoMineState {
    match state {
        AutoMineStateSaveData::Mining => AutoMineState::Mining,
        AutoMineStateSaveData::Trading => AutoMineState::Trading,
    }
}

pub(crate) fn parse_production_save_data(data: ProductionSaveData) -> ProductionFacility {
    ProductionFacility {
        modules: HashMap::from_iter(data.modules.iter().map(parse_production_module_save_data)),
    }
}

pub fn parse_shipyard_save_data(data: ShipyardSaveData) -> Shipyard {
    Shipyard {
        modules: HashMap::from_iter(data.modules.iter().map(parse_shipyard_module_save_data)),
        queue: data.queue.clone(),
    }
}

fn parse_production_module_save_data(
    data: &ProductionModuleSaveData,
) -> (ProductionModuleId, ProductionModule) {
    (
        data.module_id,
        ProductionModule {
            amount: data.amount,
            queued_recipes: data
                .queued_recipes
                .iter()
                .map(|x| ProductionQueueElement {
                    recipe: x.recipe,
                    is_repeating: x.is_repeating,
                })
                .collect(),
            running_recipes: data
                .running_recipes
                .iter()
                .map(|x| RunningProductionQueueElement {
                    recipe: x.recipe,
                    finished_at: x.finished_at,
                })
                .collect(),
        },
    )
}

pub fn parse_shipyard_module_save_data(
    data: &ShipyardModuleSaveData,
) -> (ShipyardModuleId, ShipyardModule) {
    (
        data.module_id,
        ShipyardModule {
            amount: data.amount,
            active: data
                .active
                .iter()
                .map(parse_active_shipyard_order)
                .collect(),
        },
    )
}

pub fn parse_active_shipyard_order(
    data: &ActiveShipyardOrderSaveData,
) -> OngoingShipConstructionOrder {
    OngoingShipConstructionOrder {
        ship_config: data.ship_config,
        finished_at: data.finished_at,
    }
}
