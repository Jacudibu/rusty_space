use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    AppExtStates, Commands, Condition, IntoScheduleConfigs, NextState, Res, ResMut, Resource,
    State, StateSet,
};
use bevy::prelude::{SubStates, in_state};
use bevy_egui::EguiContextPass;
use common::states::ApplicationState;
use common::types::entity_id_map::{
    AsteroidIdMap, CelestialIdMap, ConstructionSiteIdMap, GateIdMap, SectorIdMap, ShipIdMap,
    StationIdMap,
};
use persistence::data::{
    GatePairSaveData, SaveDataCollection, SectorSaveData, ShipSaveData, StationSaveData,
};

mod loading;
mod loading_gui;

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
                loading::spawn_all_sectors.run_if(in_state(LoadingState::Sectors)),
                loading::spawn_all_gates.run_if(in_state(LoadingState::Gates)),
                loading::spawn_all_stations.run_if(in_state(LoadingState::Stations)),
                loading::spawn_all_ships.run_if(in_state(LoadingState::Ships)),
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
