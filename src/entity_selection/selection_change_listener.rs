use crate::components::SelectableEntity;
use crate::entity_selection::Selected;
use crate::SpriteHandles;
use bevy::asset::Handle;
use bevy::prelude::{Added, Entity, Image, Query, RemovedComponents, Res};

pub fn on_selection_changed(
    mut selectables: Query<(&SelectableEntity, &mut Handle<Image>)>,
    new_selections: Query<Entity, Added<Selected>>,
    mut removed_selections: RemovedComponents<Selected>,
    sprite_handles: Res<SpriteHandles>,
) {
    for entity in removed_selections.read() {
        if let Ok((selectable, mut handle)) = selectables.get_mut(entity) {
            *handle = match selectable {
                SelectableEntity::Gate => sprite_handles.gate.clone(),
                SelectableEntity::Station => sprite_handles.station.clone(),
                SelectableEntity::Ship => sprite_handles.ship.clone(),
            }
        }
    }

    for entity in new_selections.iter() {
        if let Ok((selectable, mut handle)) = selectables.get_mut(entity) {
            *handle = match selectable {
                SelectableEntity::Gate => sprite_handles.gate_selected.clone(),
                SelectableEntity::Station => sprite_handles.station_selected.clone(),
                SelectableEntity::Ship => sprite_handles.ship_selected.clone(),
            }
        }
    }
}
