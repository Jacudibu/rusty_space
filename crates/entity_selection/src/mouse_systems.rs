use crate::DOUBLE_CLICK_TIME;
use crate::components::EntityIsSelected;
use crate::mouse_cursor::MouseCursor;
use crate::mouse_interaction::{LastMouseInteraction, MouseInteraction};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{
    Camera, Commands, Entity, EventReader, GlobalTransform, InheritedVisibility, MouseButton,
    Query, Real, Res, ResMut, Single, State, Time, Vec2, With, Without,
};
use camera::MainCamera;
use common::components::{RADIUS_CURSOR, SelectableEntity};
use common::constants::BevyResult;
use common::geometry;
use common::states::MouseCursorOverUiState;
use std::ops::Deref;

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_mouse_clicks(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mouse_cursor: Res<MouseCursor>,
    existing_mouse_interaction: Option<Res<MouseInteraction>>,
    last_mouse_interaction: Option<Res<LastMouseInteraction>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    selectables: Query<(
        Entity,
        &GlobalTransform,
        &SelectableEntity,
        &InheritedVisibility,
    )>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_entities: Query<Entity, With<EntityIsSelected>>,
    mouse_cursor_over_ui_state: Res<State<MouseCursorOverUiState>>,
) -> BevyResult {
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
                    return Ok(());
                }

                commands.insert_resource(MouseInteraction::new(cursor_world_pos, time.elapsed()));
                let cursor_world_pos = cursor_world_pos.extend(0.0);

                for entity in selected_entities.iter() {
                    deselect_entity(&mut commands, entity);
                }

                let first_found_entity = selectables
                    .iter()
                    .filter(|(_, _, _, visibility)| visibility == &&InheritedVisibility::VISIBLE)
                    .find(|(_, transform, selectable, _)| {
                        geometry::overlap_circle_with_circle(
                            cursor_world_pos,
                            RADIUS_CURSOR,
                            transform.translation(),
                            selectable.radius(),
                        )
                    });

                if let Some((entity, _, entity_selectable, _)) = first_found_entity {
                    if is_double_click(&time, &last_mouse_interaction) {
                        process_double_click(
                            &mut commands,
                            &selectables,
                            &camera,
                            entity_selectable,
                        )?;
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

    Ok(())
}

fn process_double_click(
    commands: &mut Commands,
    selectables: &Query<(
        Entity,
        &GlobalTransform,
        &SelectableEntity,
        &InheritedVisibility,
    )>,
    camera: &Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    entity_selectable: &SelectableEntity,
) -> BevyResult {
    let (camera, camera_transform) = camera.deref();
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
        .filter(|(_, transform, selectable, visibility)| {
            if entity_selectable != *selectable {
                return false;
            }

            if visibility != &&InheritedVisibility::VISIBLE {
                return false;
            }

            geometry::overlap_rectangle_with_circle_axis_aligned(
                left,
                right,
                bottom,
                top,
                transform.translation(),
                selectable.radius(),
            )
        })
        .for_each(|(entity, _, _, _)| {
            select_entity(commands, entity);
        });

    Ok(())
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
pub(crate) fn update_active_mouse_interaction(
    mut commands: Commands,
    mouse_interaction: Option<ResMut<MouseInteraction>>,
    mouse_cursor: Option<Res<MouseCursor>>,
    unselected_entities: Query<
        (
            Entity,
            &GlobalTransform,
            &SelectableEntity,
            &InheritedVisibility,
        ),
        Without<EntityIsSelected>,
    >,
    selected_entities: Query<(Entity, &GlobalTransform, &SelectableEntity), With<EntityIsSelected>>,
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
            .filter(|(_, transform, selectable, visibility)| {
                if visibility != &&InheritedVisibility::VISIBLE {
                    return false;
                }

                geometry::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation(),
                    selectable.radius(),
                )
            })
            .for_each(|(entity, _, _, _)| {
                select_entity(&mut commands, entity);
            });

        selected_entities
            .iter()
            .filter(|(_, transform, selectable)| {
                !geometry::overlap_rectangle_with_circle_axis_aligned(
                    left,
                    right,
                    bottom,
                    top,
                    transform.translation(),
                    selectable.radius(),
                )
            })
            .for_each(|(entity, _, _)| {
                deselect_entity(&mut commands, entity);
            });
    }
}

fn select_entity(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(EntityIsSelected {});
}

fn deselect_entity(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).remove::<EntityIsSelected>();
}
