use crate::components::{SelectableEntity, RADIUS_CURSOR};
use crate::entity_selection::mouse_interaction::{LastMouseInteraction, MouseInteraction};
use crate::entity_selection::{MouseCursor, Selected, DOUBLE_CLICK_TIME};
use crate::gui::MouseCursorOverUiState;
use crate::physics;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Commands, Entity, EventReader, GlobalTransform, MouseButton, Query, Real, Res, ResMut,
    State, Time, Transform, Vec2, With, Without,
};

#[allow(clippy::too_many_arguments)]
pub fn process_mouse_clicks(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mouse_cursor: Res<MouseCursor>,
    existing_mouse_interaction: Option<Res<MouseInteraction>>,
    last_mouse_interaction: Option<Res<LastMouseInteraction>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    selectables: Query<(Entity, &Transform, &SelectableEntity)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    selected_entities: Query<Entity, With<Selected>>,
    mouse_cursor_over_ui_state: Res<State<MouseCursorOverUiState>>,
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
                if mouse_cursor_over_ui_state.get() == &MouseCursorOverUiState::OverUI {
                    return;
                }

                commands.insert_resource(MouseInteraction::new(cursor_world_pos, time.elapsed()));
                let cursor_world_pos = cursor_world_pos.extend(0.0);

                for entity in selected_entities.iter() {
                    deselect_entity(&mut commands, entity);
                }

                let first_found_entity = selectables.iter().find(|(_, transform, selectable)| {
                    physics::overlap_circle_with_circle(
                        cursor_world_pos,
                        RADIUS_CURSOR,
                        transform.translation,
                        selectable.radius(),
                    )
                });

                if let Some((entity, _, entity_selectable)) = first_found_entity {
                    if is_double_click(&time, &last_mouse_interaction) {
                        process_double_click(
                            &mut commands,
                            &selectables,
                            &camera,
                            entity_selectable,
                        );
                    } else {
                        select_entity(&mut commands, entity);
                    }
                }
            }
            ButtonState::Released => {
                if let Some(existing_mouse_interaction) = &existing_mouse_interaction {
                    commands.insert_resource(LastMouseInteraction::from(
                        existing_mouse_interaction,
                        time.elapsed(),
                    ))
                }

                commands.remove_resource::<MouseInteraction>();
            }
        }
    }
}

fn process_double_click(
    commands: &mut Commands,
    selectables: &Query<(Entity, &Transform, &SelectableEntity)>,
    camera: &Query<(&Camera, &GlobalTransform)>,
    entity_selectable: &SelectableEntity,
) {
    let (camera, camera_transform) = camera.single();
    let rect = camera.logical_viewport_rect().unwrap();
    let offset = Vec2::new(
        camera_transform.translation().x - rect.max.x * 0.5,
        camera_transform.translation().y - rect.max.y * 0.5,
    );

    let left = rect.min.x + offset.x;
    let right = rect.max.x + offset.x;
    let bottom = rect.min.y + offset.y;
    let top = rect.max.y + offset.y;

    selectables
        .iter()
        .filter(|(_, transform, selectable)| {
            if entity_selectable != *selectable {
                return false;
            }

            physics::overlap_rectangle_with_circle_axis_aligned(
                left,
                right,
                bottom,
                top,
                transform.translation,
                selectable.radius(),
            )
        })
        .for_each(|(entity, _, _)| {
            select_entity(commands, entity);
        });
}

fn is_double_click(
    time: &Res<Time<Real>>,
    last_mouse_interaction: &Option<Res<LastMouseInteraction>>,
) -> bool {
    if let Some(last_mouse_interaction) = &last_mouse_interaction {
        last_mouse_interaction.counts_as_click
            && (time.elapsed() - last_mouse_interaction.click_end).as_millis() < DOUBLE_CLICK_TIME
    } else {
        false
    }
}

#[allow(clippy::type_complexity)]
pub fn update_active_mouse_interaction(
    mut commands: Commands,
    mouse_interaction: Option<ResMut<MouseInteraction>>,
    mouse_cursor: Option<Res<MouseCursor>>,
    unselected_entities: Query<(Entity, &Transform, &SelectableEntity), Without<Selected>>,
    selected_entities: Query<(Entity, &Transform, &SelectableEntity), With<Selected>>,
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

    if mouse_interaction.counts_as_drag() {
        let left = mouse_interaction.origin.x.min(mouse_interaction.current.x);
        let right = mouse_interaction.origin.x.max(mouse_interaction.current.x);
        let bottom = mouse_interaction.origin.y.min(mouse_interaction.current.y);
        let top = mouse_interaction.origin.y.max(mouse_interaction.current.y);

        unselected_entities
            .iter()
            .filter(|(_, transform, selectable)| {
                physics::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation,
                    selectable.radius(),
                )
            })
            .for_each(|(entity, _, _)| {
                select_entity(&mut commands, entity);
            });

        selected_entities
            .iter()
            .filter(|(_, transform, selectable)| {
                !physics::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation,
                    selectable.radius(),
                )
            })
            .for_each(|(entity, _, _)| {
                deselect_entity(&mut commands, entity);
            });
    }
}

fn select_entity(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(Selected {});
}

fn deselect_entity(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).remove::<Selected>();
}
