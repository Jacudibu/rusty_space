mod loading_gui;

use crate::builders::ship_builder::convert_behavior_save_data_to_builder_data;
use crate::builders::station_builder;
use crate::builders::station_builder::parse_shipyard_save_data;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{
    AppExtStates, Commands, Condition, IntoScheduleConfigs, NextState, Query, Res, ResMut,
    Resource, State, StateSet,
};
use bevy::prelude::{SubStates, in_state};
use bevy_egui::EguiContextPass;
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ApplicationState = ApplicationState::LoadingUniverse)]
pub(crate) enum LoadingState {
    #[default]
    Initialize,
    Sectors,
    Gates,
    Stations,
    Ships,
    Done,
}

impl LoadingState {
    /// Returns the state that should be executed after this state.
    fn next(&self) -> LoadingState {
        match self {
            LoadingState::Initialize => LoadingState::Sectors,
            LoadingState::Sectors => LoadingState::Gates,
            LoadingState::Gates => LoadingState::Stations,
            LoadingState::Stations => LoadingState::Ships,
            LoadingState::Ships => LoadingState::Done,
            LoadingState::Done => panic!("We are already done, there is no next state!"),
        }
    }
}

/// Parses save data into entities.
pub struct UniverseLoadingPlugin;
impl Plugin for UniverseLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LoadingState>();
        app.add_systems(
            Update,
            (
                init.run_if(in_state(LoadingState::Initialize)),
                spawn_all_sectors.run_if(in_state(LoadingState::Sectors)),
                spawn_all_gates.run_if(in_state(LoadingState::Gates)),
                spawn_all_stations.run_if(in_state(LoadingState::Stations)),
                spawn_all_ships.run_if(in_state(LoadingState::Ships)),
                done.run_if(in_state(LoadingState::Done)),
            ),
        );

        app.add_systems(
            EguiContextPass,
            loading_gui::display_loading_information.run_if(
                in_state(LoadingState::Sectors).or(in_state(LoadingState::Gates)
                    .or(in_state(LoadingState::Stations).or(in_state(LoadingState::Ships)))),
            ),
        );
    }
}

// Counts how many objects needs to be loaded in total in order to calculate progress.
#[derive(Resource)]
struct LoadingCounts {
    sector_count: usize,
    gate_count: usize,
    station_count: usize,
    ship_count: usize,
}

/// The first step during loading. This is where we initialize our resources.
fn init(
    mut commands: Commands,
    state: Res<State<LoadingState>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    sectors: Res<SaveDataCollection<SectorSaveData>>,
    gates: Res<SaveDataCollection<GatePairSaveData>>,
    stations: Res<SaveDataCollection<StationSaveData>>,
    ships: Res<SaveDataCollection<ShipSaveData>>,
) {
    commands.insert_resource(SectorIdMap::default());
    commands.insert_resource(AsteroidIdMap::default());
    commands.insert_resource(CelestialIdMap::default());
    commands.insert_resource(GateIdMap::default());
    commands.insert_resource(StationIdMap::default());
    commands.insert_resource(ConstructionSiteIdMap::default());
    commands.insert_resource(ShipIdMap::default());

    commands.insert_resource(LoadingCounts {
        sector_count: sectors.data.len(),
        gate_count: gates.data.len(),
        station_count: stations.data.len(),
        ship_count: ships.data.len(),
    });

    next_state.set(state.next());
}

/// The final step during loading.
/// We are cleaning up any temporary data that's been constructed during the loading process here.
fn done(mut commands: Commands, mut next_state: ResMut<NextState<ApplicationState>>) {
    commands.remove_resource::<LoadingCounts>();

    commands.remove_resource::<SaveDataCollection<SectorSaveData>>();
    commands.remove_resource::<SaveDataCollection<GatePairSaveData>>();
    commands.remove_resource::<SaveDataCollection<StationSaveData>>();
    commands.remove_resource::<SaveDataCollection<ShipSaveData>>();

    next_state.set(ApplicationState::InGame);
}

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
        .map(station_builder::parse_production_save_data);
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
