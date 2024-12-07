use crate::components::SelectableEntity;
use crate::entity_selection::Selected;
use crate::SpriteHandles;
use bevy::prelude::{Added, Entity, Query, RemovedComponents, Res, Sprite};

pub fn on_selection_changed(
    mut selectables: Query<(&SelectableEntity, &mut Sprite)>,
    new_selections: Query<Entity, Added<Selected>>,
    mut removed_selections: RemovedComponents<Selected>,
    sprite_handles: Res<SpriteHandles>,
) {
    for entity in removed_selections.read() {
        if let Ok((selectable, mut sprite)) = selectables.get_mut(entity) {
            sprite.image = match selectable {
                SelectableEntity::Asteroid => sprite_handles.asteroid.clone(),
                SelectableEntity::Gate => sprite_handles.gate.clone(),
                SelectableEntity::Planet => sprite_handles.planet.clone(),
                SelectableEntity::Ship => sprite_handles.ship.clone(),
                SelectableEntity::Star => sprite_handles.star.clone(),
                SelectableEntity::Station => sprite_handles.station.clone(),
            }
        }
    }

    for entity in new_selections.iter() {
        if let Ok((selectable, mut sprite)) = selectables.get_mut(entity) {
            sprite.image = match selectable {
                SelectableEntity::Asteroid => sprite_handles.asteroid_selected.clone(),
                SelectableEntity::Gate => sprite_handles.gate_selected.clone(),
                SelectableEntity::Planet => sprite_handles.planet_selected.clone(),
                SelectableEntity::Ship => sprite_handles.ship_selected.clone(),
                SelectableEntity::Star => sprite_handles.star_selected.clone(),
                SelectableEntity::Station => sprite_handles.station_selected.clone(),
            }
        }
    }
}
