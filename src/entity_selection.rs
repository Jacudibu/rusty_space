use crate::components::SelectableEntity;
use crate::mouse_cursor::MouseCursor;
use crate::{physics, SpriteHandles};
use bevy::asset::Handle;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::Rot2;
use bevy::prelude::{
    Added, Camera, Commands, Component, Entity, Event, EventReader, GizmoConfigGroup, Gizmos,
    GlobalTransform, Image, MouseButton, Query, Reflect, RemovedComponents, Res, ResMut, Resource,
    Time, Transform, Vec2, Window, With, Without,
};
use std::time::Duration;

#[derive(Component)]
pub struct Selected {}

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
    total_travel: f32,
}

impl MouseInteraction {
    pub fn new(position: Vec2, start: Duration) -> Self {
        Self {
            origin: position,
            current: position,
            start,
            total_travel: 0.0,
        }
    }

    pub fn update(&mut self, new_pos: Vec2) {
        self.total_travel += self.current.distance(new_pos);
        self.current = new_pos;
    }

    pub fn counts_as_click(&self, current_time: Duration) -> bool {
        // The average mouse click lasts about 85 milliseconds
        (current_time - self.start).as_millis() < 100
    }

    pub fn counts_as_drag(&self, current_time: Duration) -> bool {
        // TODO: Should make this depend on zoom level
        // Using logical positions might also be an option, but that would exclude camera movement through
        !self.counts_as_click(current_time) && self.total_travel > 1.0
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

    if !mouse_interaction.counts_as_drag(time.elapsed()) {
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
    mut commands: Commands,
    mouse_interaction: Option<ResMut<MouseInteraction>>,
    mouse_cursor: Option<Res<MouseCursor>>,
    unselected_entities: Query<(Entity, &Transform), (With<SelectableEntity>, Without<Selected>)>,
    selected_entities: Query<(Entity, &Transform), (With<SelectableEntity>, With<Selected>)>,
    time: Res<Time>,
) {
    let Some(mut mouse_interaction) = mouse_interaction else {
        return;
    };

    let Some(mouse_cursor) = mouse_cursor else {
        return;
    };

    if let Some(world_space) = mouse_cursor.world_space {
        mouse_interaction.update(world_space);
    }

    if mouse_interaction.counts_as_drag(time.elapsed()) {
        let left = mouse_interaction.origin.x.min(mouse_interaction.current.x);
        let right = mouse_interaction.origin.x.max(mouse_interaction.current.x);
        let bottom = mouse_interaction.origin.y.min(mouse_interaction.current.y);
        let top = mouse_interaction.origin.y.max(mouse_interaction.current.y);

        unselected_entities
            .iter()
            .filter(|(_, transform)| {
                physics::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation,
                    RADIUS * RADIUS,
                )
            })
            .for_each(|(entity, _)| {
                commands.entity(entity).insert(Selected {});
            });

        selected_entities
            .iter()
            .filter(|(_, transform)| {
                !physics::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation,
                    RADIUS * RADIUS,
                )
            })
            .for_each(|(entity, _)| {
                commands.entity(entity).remove::<Selected>();
            });
    }
}

pub fn process_mouse_clicks(
    mut commands: Commands,
    time: Res<Time>,
    mouse_cursor: Res<MouseCursor>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    selectables: Query<(Entity, &Transform), With<SelectableEntity>>,
    selected_entities: Query<Entity, With<Selected>>,
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
                let cursor_world_pos = cursor_world_pos.extend(0.0);

                for entity in selected_entities.iter() {
                    commands.entity(entity).remove::<Selected>();
                }

                selectables
                    .iter()
                    .filter(|(_, transform)| {
                        physics::overlap_circle_with_circle(
                            cursor_world_pos,
                            RADIUS,
                            transform.translation,
                            RADIUS,
                        )
                    })
                    .for_each(|(entity, _)| {
                        commands.entity(entity).insert(Selected {});
                    });
            }
            ButtonState::Released => {
                commands.remove_resource::<MouseInteraction>();
            }
        }
    }
}

pub fn on_selection_changed(
    mut selectables: Query<(&SelectableEntity, &mut Handle<Image>)>,
    new_selections: Query<Entity, Added<Selected>>,
    mut removed_selections: RemovedComponents<Selected>,
    sprite_handles: Res<SpriteHandles>,
) {
    for entity in removed_selections.read() {
        if let Ok((selectable, mut handle)) = selectables.get_mut(entity) {
            *handle = match selectable {
                SelectableEntity::Station => sprite_handles.station.clone(),
                SelectableEntity::Ship => sprite_handles.ship.clone(),
            }
        }
    }

    for entity in new_selections.iter() {
        if let Ok((selectable, mut handle)) = selectables.get_mut(entity) {
            *handle = match selectable {
                SelectableEntity::Station => sprite_handles.station_selected.clone(),
                SelectableEntity::Ship => sprite_handles.ship_selected.clone(),
            }
        }
    }
}
