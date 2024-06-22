use crate::components::SelectableEntity;
use crate::mouse_cursor::MouseCursor;
use crate::SpriteHandles;
use bevy::asset::Handle;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::Rot2;
use bevy::prelude::{
    Camera, Commands, Entity, Event, EventReader, EventWriter, GizmoConfigGroup, Gizmos,
    GlobalTransform, Image, MouseButton, Query, Reflect, Res, ResMut, Resource, Time, Transform,
    Vec2, Window, With,
};
use std::time::Duration;

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

#[derive(Resource)]
pub struct MouseInteraction {
    pub origin: Vec2,
    pub current: Vec2,
    pub start: Duration,
}

impl MouseInteraction {
    pub fn new(position: Vec2, start: Duration) -> Self {
        Self {
            origin: position,
            current: position,
            start,
        }
    }

    pub fn distance(&self) -> f32 {
        self.origin.distance(self.current)
    }

    pub fn counts_as_click(&self, current_time: Duration) -> bool {
        // The average mouse click lasts about 85 milliseconds
        self.distance() < 0.1 && (current_time - self.start).as_millis() > 100
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MouseInteractionGizmos;

const RADIUS: f32 = 8.0;
pub fn draw_mouse_interactions(
    mut gizmos: Gizmos<MouseInteractionGizmos>,
    mouse_interaction: Option<Res<MouseInteraction>>,
    time: Res<Time>,
) {
    let Some(mouse_interaction) = mouse_interaction else {
        return;
    };

    if mouse_interaction.counts_as_click(time.elapsed()) {
        return;
    }

    let size = mouse_interaction.origin - mouse_interaction.current;
    let origin = mouse_interaction.origin - size * 0.5;

    gizmos.rect_2d(
        origin,
        Rot2::default(),
        size,
        bevy::color::palettes::css::YELLOW_GREEN,
    );
}

pub fn update_mouse_interaction(
    mut mouse_interaction: Option<ResMut<MouseInteraction>>,
    mouse_cursor: Option<Res<MouseCursor>>,
    mut event_writer: EventWriter<SelectionChangedEvent>,
    selectables: Query<(Entity, &Transform), With<SelectableEntity>>,
    time: Res<Time>,
) {
    let Some(mut mouse_interaction) = mouse_interaction else {
        return;
    };

    let Some(mouse_cursor) = mouse_cursor else {
        return;
    };

    if let Some(world_space) = mouse_cursor.world_space {
        mouse_interaction.current = world_space;
    }

    let entities = if mouse_interaction.counts_as_click(time.elapsed()) {
        // TODO: consider doing this in select_entities on pressed, and just doing nothing here.
        // Overlap Point
        selectables
            .iter()
            .filter_map(|(entity, transform)| {
                let x = mouse_interaction.current.x - transform.translation.x;
                let y = mouse_interaction.current.y - transform.translation.y;

                if x * x + y * y <= RADIUS * RADIUS {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect()
    } else {
        // Overlap Rectangle
        let left = mouse_interaction.origin.x.min(mouse_interaction.current.x);
        let right = mouse_interaction.origin.x.max(mouse_interaction.current.x);
        let bottom = mouse_interaction.origin.y.min(mouse_interaction.current.y);
        let top = mouse_interaction.origin.y.max(mouse_interaction.current.y);

        selectables
            .iter()
            .filter_map(|(entity, transform)| {
                let closest_x = transform.translation.x.max(left).min(right);
                let closest_y = transform.translation.y.max(bottom).min(top);

                let distance = (transform.translation.x - closest_x).powi(2)
                    + (transform.translation.y - closest_y).powi(2);

                if distance <= RADIUS * RADIUS {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect()
    };

    event_writer.send(SelectionChangedEvent {
        new_selection: entities,
    });
}

pub fn select_entities(
    mut commands: Commands,
    time: Res<Time>,
    mouse_cursor: Res<MouseCursor>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
) {
    for event in mouse_button_events.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        let Some(cursor_world_pos) = mouse_cursor.world_space else {
            continue;
        };

        match event.state {
            ButtonState::Pressed => {
                commands.insert_resource(MouseInteraction::new(cursor_world_pos, time.elapsed()));
                // TODO: Clear previous selection
            }
            ButtonState::Released => {
                commands.remove_resource::<MouseInteraction>();
            }
        }
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
