use crate::SpriteHandles;
use crate::components::SelectableEntity;
use crate::entity_selection::IsEntitySelected;
use crate::game_data::AsteroidManifest;
use crate::session_data::ShipConfigurationManifest;
use bevy::prelude::{Added, Entity, Query, RemovedComponents, Res, Sprite};

pub fn on_selection_changed(
    mut selectables: Query<(&SelectableEntity, &mut Sprite)>,
    new_selections: Query<Entity, Added<IsEntitySelected>>,
    mut removed_selections: RemovedComponents<IsEntitySelected>,
    asteroid_manifest: Res<AsteroidManifest>,
    ship_configs: Res<ShipConfigurationManifest>,
    sprite_handles: Res<SpriteHandles>,
) {
    for entity in removed_selections.read() {
        if let Ok((selectable, mut sprite)) = selectables.get_mut(entity) {
            sprite.image = match selectable {
                SelectableEntity::Asteroid(id) => {
                    asteroid_manifest.get_by_ref(id).unwrap().sprite.clone()
                }
                SelectableEntity::Gate => sprite_handles.gate.clone(),
                SelectableEntity::Celestial => sprite_handles.planet.clone(),
                SelectableEntity::Ship(id) => ship_configs.get_by_id(id).unwrap().sprite.clone(),
                SelectableEntity::Star => sprite_handles.star.clone(),
                SelectableEntity::Station => sprite_handles.station.clone(),
            }
        }
    }

    for entity in new_selections.iter() {
        if let Ok((selectable, mut sprite)) = selectables.get_mut(entity) {
            sprite.image = match selectable {
                SelectableEntity::Asteroid(id) => asteroid_manifest
                    .get_by_ref(id)
                    .unwrap()
                    .sprite_selected
                    .clone(),
                SelectableEntity::Gate => sprite_handles.gate_selected.clone(),
                SelectableEntity::Celestial => sprite_handles.planet_selected.clone(),
                SelectableEntity::Ship(id) => {
                    ship_configs.get_by_id(id).unwrap().sprite_selected.clone()
                }
                SelectableEntity::Star => sprite_handles.star_selected.clone(),
                SelectableEntity::Station => sprite_handles.station_selected.clone(),
            }
        }
    }
}
