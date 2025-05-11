use crate::builders::ship_builder::convert_behavior_save_data_to_builder_data;
use crate::builders::station_builder;
use crate::builders::station_builder::parse_shipyard_save_data;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{
    AppExtStates, Commands, IntoScheduleConfigs, NextState, Query, Res, ResMut, StateSet,
};
use bevy::prelude::{SubStates, in_state};
use common::components::celestials::Celestial;
use common::components::ship_velocity::ShipVelocity;
use common::components::{BuyOrders, Sector, SectorWithCelestials};
use common::game_data::{AsteroidManifest, ItemManifest, RecipeManifest};
use common::session_data::ShipConfigurationManifest;
use common::states::ApplicationState;
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
    GatePairSaveData, SaveDataCollection, SectorSaveData, ShipSaveData, StationSaveData,
};

/// Parses save data into entities.
pub struct UniverseLoadingPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ApplicationState = ApplicationState::LoadingUniverse)]
pub(crate) enum LoadingState {
    #[default]
    Sectors,
    Gates,
    Stations,
    Ships,
    Done,
}

impl Plugin for UniverseLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LoadingState>();
        app.add_systems(
            Update,
            (
                spawn_all_sectors.run_if(in_state(LoadingState::Sectors)),
                spawn_all_gates.run_if(in_state(LoadingState::Gates)),
                spawn_all_stations.run_if(in_state(LoadingState::Stations)),
                spawn_all_ships.run_if(in_state(LoadingState::Ships)),
                done.run_if(in_state(LoadingState::Done)),
            ),
        );
    }
}

fn done(mut next_state: ResMut<NextState<ApplicationState>>) {
    next_state.set(ApplicationState::InGame);
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllSectorArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    map_layout: Res<'w, MapLayout>,
    asteroid_manifest: Res<'w, AsteroidManifest>,
}

pub(crate) fn spawn_all_sectors(
    data: Res<SaveDataCollection<SectorSaveData>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllSectorArgs,
) {
    let mut sector_id_map = SectorIdMap::default();
    let mut asteroid_id_map = AsteroidIdMap::default();
    let mut planet_id_map = CelestialIdMap::default();
    for builder in &data.data {
        let coordinate = builder.coordinate;
        let entity = spawn_sector(
            &mut args.commands,
            &args.map_layout.hex_layout,
            builder.coordinate,
            &builder.features,
            &args.sprites,
            &mut asteroid_id_map,
            &mut planet_id_map,
            &args.asteroid_manifest,
        );
        sector_id_map.insert(coordinate, entity);
    }

    args.commands
        .remove_resource::<SaveDataCollection<SectorSaveData>>();
    args.commands.insert_resource(sector_id_map);
    args.commands.insert_resource(asteroid_id_map);
    args.commands.insert_resource(planet_id_map);
    next_state.set(LoadingState::Gates);
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllGatesArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorWithCelestials>)>,
    celestials: Query<'w, 's, &'static Celestial>,

    sector_id_map: Res<'w, SectorIdMap>,
}

pub(crate) fn spawn_all_gates(
    data: Res<SaveDataCollection<GatePairSaveData>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllGatesArgs,
) {
    let mut gate_id_map = GateIdMap::new();

    for data in &data.data {
        spawn_gate_pair(
            &mut args.commands,
            &mut gate_id_map,
            &mut args.sectors,
            &args.sprites,
            data.from_id,
            data.from_position.to_sector_position(&args.sector_id_map),
            data.to_id,
            data.to_position.to_sector_position(&args.sector_id_map),
        )
    }

    args.commands
        .remove_resource::<SaveDataCollection<GatePairSaveData>>();
    args.commands.insert_resource(gate_id_map);
    next_state.set(LoadingState::Stations);
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
}

pub(crate) fn spawn_all_stations(
    data: Res<SaveDataCollection<StationSaveData>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllStationsArgs,
) {
    let mut station_id_map = StationIdMap::new();
    let mut construction_site_id_map = ConstructionSiteIdMap::new();
    for data in &data.data {
        build(
            data,
            &mut args,
            &mut station_id_map,
            &mut construction_site_id_map,
        );
    }

    args.commands
        .remove_resource::<SaveDataCollection<StationSaveData>>();
    args.commands.insert_resource(station_id_map);
    args.commands.insert_resource(construction_site_id_map);

    next_state.set(LoadingState::Ships);
}

fn build(
    data: &StationSaveData,
    args: &mut SpawnAllStationsArgs,
    station_id_map: &mut StationIdMap,
    construction_site_id_map: &mut ConstructionSiteIdMap,
) {
    let sector_entity = args
        .sector_id_map
        .get_entity(&data.position.sector)
        .unwrap();

    // TODO: All custom trade data is lost right now
    let buys = data
        .buy_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());
    let sells = data
        .sell_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());

    let production = data
        .production_modules
        .clone() // TODO: Can we get rid of those clones?
        .map(station_builder::parse_production_save_data);
    let shipyard = data.shipyard_modules.clone().map(parse_shipyard_save_data);

    let data = StationSpawnData {
        id: data.id,
        name: data.name.clone(),
        sector_position: SectorPosition {
            sector: *sector_entity,
            local_position: data.position.local_position,
        },
        shipyard,
        production,
        buys,
        sells,
        construction_site: data.construction_site.clone().map(|x| {
            ConstructionSiteSpawnData::new(x.queue, BuyOrders::default()) // TODO
                .with_progress(x.current_progress)
        }),
    };

    spawn_station(
        &mut args.commands,
        &mut args.sectors,
        station_id_map,
        construction_site_id_map,
        &args.sprites,
        &args.items,
        &args.recipes,
        data,
    );
}

#[derive(SystemParam)]
pub(crate) struct SpawnAllShipsArgs<'w, 's> {
    commands: Commands<'w, 's>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,
}

pub(crate) fn spawn_all_ships(
    data: Res<SaveDataCollection<ShipSaveData>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    mut args: SpawnAllShipsArgs,
) {
    let mut ship_id_map = ShipIdMap::new();
    for data in &data.data {
        spawn_ship(
            &mut args.commands,
            data.id,
            data.name.clone(),
            &mut args.sectors,
            args.sector_id_map.id_to_entity()[&data.position.sector],
            data.position.local_position,
            data.rotation_degrees,
            ShipVelocity {
                forward: data.forward_velocity,
                angular: data.angular_velocity,
            },
            convert_behavior_save_data_to_builder_data(data.behavior),
            &mut ship_id_map,
            args.ship_configurations.get_by_id(&data.config_id).unwrap(),
        );
    }

    args.commands
        .remove_resource::<SaveDataCollection<ShipSaveData>>();
    args.commands.insert_resource(ship_id_map);

    next_state.set(LoadingState::Done);
}
