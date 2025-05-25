use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, EventWriter, KeyCode, MouseButton, Plugin, Query, Res, Update, With};
use common::components::task_queue::TaskQueue;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskInsertionMode};
use common::types::map_layout::MapLayout;
use common::types::ship_tasks::MoveToPosition;
use entity_selection::components::EntityIsSelected;
use entity_selection::mouse_cursor::MouseCursor;

/// Adds ways through which the user can send command-events to control ships manually.
/// The same events may be used by AI.
pub struct ShipControllerPlugin;
impl Plugin for ShipControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InsertTaskIntoQueueCommand<MoveToPosition>>();
        app.add_systems(Update, send_move_command);
    }
}

/// This [System] sends a move command when the user right clicks into empty space whilst having entities with a task queue selected.
pub(crate) fn send_move_command(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selected_ships: Query<Entity, (With<EntityIsSelected>, With<TaskQueue>)>,
    mouse_cursor: Res<MouseCursor>,
    mut event_writer: EventWriter<InsertTaskIntoQueueCommand<MoveToPosition>>,
    map_layout: Res<MapLayout>,
) {
    if !mouse_input.just_released(MouseButton::Right) {
        return;
    }

    let Some(position) = &mouse_cursor.sector_space else {
        return;
    };

    let global_position = position.sector_position.local_position
        + map_layout.hex_layout.hex_to_world_pos(position.coordinates);

    event_writer.write_batch(
        selected_ships
            .iter()
            .map(|entity| InsertTaskIntoQueueCommand::<MoveToPosition> {
                entity,
                task_data: MoveToPosition {
                    sector_position: position.sector_position,
                    global_position,
                },
                insertion_mode: if keyboard_input.pressed(KeyCode::ControlLeft) {
                    TaskInsertionMode::Prepend
                } else {
                    TaskInsertionMode::Append
                },
            }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ButtonInput;
    use bevy::prelude::{MouseButton, Vec2};
    use common::types::entity_wrappers::SectorEntity;
    use common::types::sector_position::SectorPosition;
    use entity_selection::components::EntityIsSelected;
    use entity_selection::mouse_cursor::{MouseCursor, MouseSectorPosition};
    use hexx::Hex;
    use test_utils::test_events;

    fn build_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(ShipControllerPlugin);
        app.insert_resource(ButtonInput::<MouseButton>::default());
        app.insert_resource(MouseCursor::default());
        app
    }

    #[test]
    fn right_clicking_with_selected_ship_should_send_move_command() {
        let mut app = build_test_app();

        let entity = app
            .world_mut()
            .spawn((TaskQueue::default(), EntityIsSelected {}))
            .id();

        let sector_position = SectorPosition {
            local_position: Vec2::new(50.0, 50.0),
            sector: SectorEntity::from(Entity::from_raw(42)),
        };

        app.world_mut().resource_mut::<MouseCursor>().sector_space = Some(MouseSectorPosition {
            coordinates: Hex::default(),
            sector_position,
        });
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);

        app.update();

        test_events::<InsertTaskIntoQueueCommand<MoveToPosition>, _>(&mut app, |mut events| {
            assert!(events.next().is_none())
        });

        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .release(MouseButton::Right);

        app.update();

        test_events::<InsertTaskIntoQueueCommand<MoveToPosition>, _>(&mut app, |mut events| {
            let event = events.next().unwrap();
            assert_eq!(event.entity, entity);
            assert_eq!(
                event.task_data.sector_position.local_position,
                sector_position.local_position
            );
            assert_eq!(
                event.task_data.sector_position.sector,
                sector_position.sector
            );
        });
    }
}
