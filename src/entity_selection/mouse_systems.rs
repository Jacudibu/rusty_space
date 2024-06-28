use crate::components::SelectableEntity;
use crate::entity_selection::gizmos::RADIUS;
use crate::entity_selection::mouse_interaction::{LastMouseInteraction, MouseInteraction};
use crate::entity_selection::{MouseCursor, Selected, DOUBLE_CLICK_TIME};
use crate::gui::MouseCursorOverUiState;
use crate::physics;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Commands, Entity, EventReader, MouseButton, Query, Real, Res, ResMut, State, Time, Transform,
    With, Without,
};
use bevy::render::primitives::Frustum;

#[allow(clippy::too_many_arguments)]
pub fn process_mouse_clicks(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mouse_cursor: Res<MouseCursor>,
    existing_mouse_interaction: Option<Res<MouseInteraction>>,
    last_mouse_interaction: Option<Res<LastMouseInteraction>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    selectables: Query<(Entity, &Transform, &SelectableEntity)>,
    camera: Query<&Frustum>,
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
                    commands.entity(entity).remove::<Selected>();
                }

                let first_found_entity = selectables.iter().find(|(_, transform, _)| {
                    physics::overlap_circle_with_circle(
                        cursor_world_pos,
                        RADIUS,
                        transform.translation,
                        RADIUS,
                    )
                });

                if let Some((entity, _, entity_selectable)) = first_found_entity {
                    if is_double_click(&time, &last_mouse_interaction) {
                        selectables
                            .iter()
                            .filter(|(_, transform, selectable)| {
                                if entity_selectable != *selectable {
                                    return false;
                                }

                                // There's probably a fancier way of doing this
                                // It also doesn't work with sectors, need to swap to GlobalTransform here
                                camera.single().intersects_sphere(
                                    &bevy::render::primitives::Sphere {
                                        radius: RADIUS,
                                        center: transform.translation.into(),
                                    },
                                    false,
                                )
                            })
                            .for_each(|(entity, _, _)| {
                                commands.entity(entity).insert(Selected {});
                            });
                    } else {
                        commands.entity(entity).insert(Selected {});
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
    unselected_entities: Query<(Entity, &Transform), (With<SelectableEntity>, Without<Selected>)>,
    selected_entities: Query<(Entity, &Transform), (With<SelectableEntity>, With<Selected>)>,
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
