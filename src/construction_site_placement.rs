use crate::components::{SectorComponent, SectorStarComponent, StarComponent};
use crate::entity_selection::MouseCursor;
use crate::game_data::{
    ConstructableModuleId, ItemManifest, RecipeManifest, SILICA_PRODUCTION_MODULE_ID,
};
use crate::persistence::{ConstructionSiteIdMap, StationIdMap};
use crate::utils::entity_spawners::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use crate::{SpriteHandles, constants};
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::{
    AppExtStates, Color, Commands, Component, Entity, IntoSystemConfigs, KeyCode, LinearRgba,
    MouseButton, Name, NextState, OnEnter, OnExit, Query, Res, ResMut, State, States, Transform,
    Update, With, in_state,
};
use bevy::sprite::Sprite;

/// Plugin for placing new Construction Sites.
pub struct ConstructionSitePlacementPlugin;
impl Plugin for ConstructionSitePlacementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConstructionMode::Off);
        app.add_systems(
            OnEnter(ConstructionMode::On),
            spawn_construction_preview_entity,
        );
        app.add_systems(
            OnExit(ConstructionMode::On),
            despawn_construction_preview_entity,
        );
        app.add_systems(
            Update,
            (
                toggle_construction_mode,
                (move_preview_entity, create_construction_site_on_mouse_click)
                    .run_if(in_state(ConstructionMode::On)),
            ),
        );
    }
}

/// States indicating whether we are currently in Construction Mode.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionMode {
    Off,
    On,
}

/// Toggles the [ConstructionMode] between [ConstructionMode::On] and [ConstructionMode::Off] when pressing a button.
fn toggle_construction_mode(
    state: Res<State<ConstructionMode>>,
    mut next_state: ResMut<NextState<ConstructionMode>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::KeyV) {
        return;
    }

    match state.get() {
        ConstructionMode::Off => {
            next_state.set(ConstructionMode::On);
        }
        ConstructionMode::On => {
            next_state.set(ConstructionMode::Off);
        }
    }
}

/// Marker component for the construction preview entity
#[derive(Component)]
struct PreviewEntityComponent {}

fn spawn_construction_preview_entity(mut commands: Commands, sprites: Res<SpriteHandles>) {
    commands.spawn((
        Name::new("Construction Site Preview"),
        PreviewEntityComponent {},
        Sprite {
            image: sprites.station.clone(),
            color: Color::LinearRgba(LinearRgba::new(0.0, 1.0, 0.0, 0.75)),
            ..Default::default()
        },
    ));
}

/// Despawns all entities marked with a [PreviewEntityComponent]
fn despawn_construction_preview_entity(
    mut commands: Commands,
    query: Query<Entity, With<PreviewEntityComponent>>,
) {
    for x in query.iter() {
        commands.entity(x).despawn();
    }
}

/// Moves all entities marked with a [PreviewEntityComponent] to the mouse cursor.
fn move_preview_entity(
    mouse_cursor: Res<MouseCursor>,
    mut query: Query<&mut Transform, With<PreviewEntityComponent>>,
) {
    let Some(position) = mouse_cursor.world_space else {
        return;
    };

    if mouse_cursor.sector_space.is_none() {
        return;
    }

    for mut transform in query.iter_mut() {
        transform.translation = position.extend(constants::z_layers::TRANSPARENT_PREVIEW_ITEM);
    }
}

/// Creates a new Construction Site when the player clicks the left mouse button.
#[allow(clippy::too_many_arguments)]
fn create_construction_site_on_mouse_click(
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
