use crate::SpriteHandles;
use crate::components::{SectorComponent, SectorStarComponent, StarComponent};
use crate::entity_selection::MouseCursor;
use crate::game_data::{
    ConstructableModuleId, ItemManifest, RecipeManifest, SILICA_PRODUCTION_MODULE_ID,
};
use crate::persistence::{ConstructionSiteIdMap, StationIdMap};
use crate::utils::entity_spawners::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::{
    AppExtStates, Commands, IntoSystemConfigs, KeyCode, MouseButton, NextState, OnEnter, OnExit,
    Query, Res, ResMut, State, States, Update, in_state,
};

pub struct ConstructionSitePlacementPlugin;
impl Plugin for ConstructionSitePlacementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConstructionPreviewState::Off);
        app.add_systems(OnEnter(ConstructionPreviewState::On), spawn_preview_entity);
        app.add_systems(OnExit(ConstructionPreviewState::On), despawn_preview_entity);
        app.add_systems(
            Update,
            (
                toggle_construction_site_placement,
                (
                    move_preview_entity,
                    create_construction_site_on_button_press,
                )
                    .run_if(in_state(ConstructionPreviewState::On)),
            ),
        );
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionPreviewState {
    Off,
    On,
}

pub fn toggle_construction_site_placement(
    state: Res<State<ConstructionPreviewState>>,
    mut next_state: ResMut<NextState<ConstructionPreviewState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::KeyV) {
        return;
    }

    match state.get() {
        ConstructionPreviewState::Off => {
            next_state.set(ConstructionPreviewState::On);
        }
        ConstructionPreviewState::On => {
            next_state.set(ConstructionPreviewState::Off);
        }
    }
}

pub fn spawn_preview_entity() {
    // TODO
}

pub fn despawn_preview_entity() {
    // TODO
}

pub fn move_preview_entity() {
    // TODO
}

#[allow(clippy::too_many_arguments)]
pub fn create_construction_site_on_button_press(
    mut commands: Commands,
    mut sector_query: Query<(&mut SectorComponent, Option<&SectorStarComponent>)>,
    mut station_id_map: ResMut<StationIdMap>,
    mut construction_site_id_map: ResMut<ConstructionSiteIdMap>,
    stars: Query<&StarComponent>,
    sprites: Res<SpriteHandles>,
    item_manifest: Res<ItemManifest>,
    recipe_manifest: Res<RecipeManifest>,
    mouse_cursor: Res<MouseCursor>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // TODO: This should fire an event, which is then either processed locally or sent to the server
    let Some(sector_pos) = &mouse_cursor.sector_space else {
        return;
    };

    // TODO: Test whether position is valid: Too close to other stations or sector edge
    // TODO: Snap position to nearby orbits to avoid collisions / overlaps

    let construction_site =
        ConstructionSiteSpawnData::new(vec![ConstructableModuleId::ProductionModule(
            SILICA_PRODUCTION_MODULE_ID,
        )]);
    let data = StationSpawnData::new(
        "Fancy Station",
        construction_site,
        sector_pos.sector_position,
    );

    spawn_station(
        &mut commands,
        &mut sector_query,
        &stars,
        &mut station_id_map,
        &mut construction_site_id_map,
        &sprites,
        &item_manifest,
        &recipe_manifest,
        data,
    );
}
