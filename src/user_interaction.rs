use crate::SpriteHandles;
use crate::components::SectorComponent;
use crate::entity_selection::MouseCursor;
use crate::game_data::{
    ConstructableModuleId, ItemManifest, RecipeManifest, SILICA_PRODUCTION_MODULE_ID,
};
use crate::persistence::{ConstructionSiteIdMap, StationIdMap};
use crate::utils::entity_spawners::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, KeyCode, Query, Res, ResMut, Update};

pub struct UserInteraction;
impl Plugin for UserInteraction {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_construction_site_on_button_press);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_construction_site_on_button_press(
    mut commands: Commands,
    mut sector_query: Query<&mut SectorComponent>,
    mut station_id_map: ResMut<StationIdMap>,
    mut construction_site_id_map: ResMut<ConstructionSiteIdMap>,
    sprites: Res<SpriteHandles>,
    item_manifest: Res<ItemManifest>,
    recipe_manifest: Res<RecipeManifest>,
    mouse_cursor: Res<MouseCursor>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::KeyV) {
        // TODO: This should fire an event, which is then either processed locally or sent to the server
        return;
    }

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
        &mut station_id_map,
        &mut construction_site_id_map,
        &sprites,
        &item_manifest,
        &recipe_manifest,
        data,
    );
}
