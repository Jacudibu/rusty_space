use crate::components::SelectableEntity;
use crate::mouse_cursor::MouseCursor;
use crate::SpriteHandles;
use bevy::asset::Handle;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Commands, Entity, Event, EventReader, EventWriter, GlobalTransform, Image, MouseButton,
    Query, Res, ResMut, Resource, Transform, Window, With,
};

#[derive(Event)]
pub struct SelectionChangedEvent {
    pub new_selection: Vec<Entity>,
}

#[derive(Resource)]
pub struct SelectedEntities {
    pub entities: Vec<Entity>,
}

pub fn update_cursor_position(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<MouseCursor>,
) {
    if let Some(position) = windows.single().cursor_position() {
        let (camera, transform) = camera.single();
        let world_pos = camera.viewport_to_world_2d(transform, position);

        cursor.screen_space = Some(position);
        cursor.world_space = world_pos;
    } else {
        cursor.screen_space = None;
        cursor.world_space = None;
    }
}

pub fn select_entities(
    selectables: Query<(Entity, &Transform), With<SelectableEntity>>,
    mouse_cursor: Res<MouseCursor>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut event_writer: EventWriter<SelectionChangedEvent>,
) {
    const RADIUS: f32 = 8.0;

    for event in mouse_button_events.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        // TODO: Multi Selection and stuff
        if event.state != ButtonState::Pressed {
            continue;
        }

        let Some(cursor_world_pos) = mouse_cursor.world_space else {
            continue;
        };

        let entities: Vec<Entity> = selectables
            .iter()
            .filter_map(|(entity, transform)| {
                let x = cursor_world_pos.x - transform.translation.x;
                let y = cursor_world_pos.y - transform.translation.y;

                if x * x + y * y <= RADIUS * RADIUS {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect();

        event_writer.send(SelectionChangedEvent {
            new_selection: entities,
        });
    }
}

pub fn on_selection_changed(
    mut commands: Commands,
    mut selectables: Query<(&SelectableEntity, &mut Handle<Image>)>,
    old_selection: Option<Res<SelectedEntities>>,
    mut event: EventReader<SelectionChangedEvent>,
    sprite_handles: Res<SpriteHandles>,
) {
    let Some(event) = event.read().last() else {
        return;
    };

    if old_selection.is_none() && event.new_selection.is_empty() {
        return;
    }

    if let Some(old_selection) = old_selection {
        for entity in &old_selection.entities {
            if let Ok((selectable, mut handle)) = selectables.get_mut(*entity) {
                *handle = match selectable {
                    SelectableEntity::Station => sprite_handles.station.clone(),
                    SelectableEntity::Ship => sprite_handles.ship.clone(),
                }
            }
        }
    }

    if event.new_selection.is_empty() {
        commands.remove_resource::<SelectedEntities>();
        return;
    }

    for entity in &event.new_selection {
        if let Ok((selectable, mut handle)) = selectables.get_mut(*entity) {
            *handle = match selectable {
                SelectableEntity::Station => sprite_handles.station_selected.clone(),
                SelectableEntity::Ship => sprite_handles.ship_selected.clone(),
            }
        }
    }

    commands.insert_resource(SelectedEntities {
        entities: event.new_selection.clone(),
    })
}
