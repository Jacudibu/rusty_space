use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, Event, EventWriter, MouseButton, Plugin, Query, Res, Update, With};
use common::components::Ship;
use common::types::sector_position::SectorPosition;
use entity_selection::components::EntityIsSelected;
use entity_selection::mouse_cursor::MouseCursor;

/// Adds ways through which the user can send command-events to control ships manually.
/// The same events may be used by AI.
pub struct ShipControllerPlugin;
impl Plugin for ShipControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveShipToPositionCommand>();
        app.add_systems(Update, send_move_command);
    }
}

/// This [System] sends a move command when the user right clicks into empty space.
pub(crate) fn send_move_command(
    input: Res<ButtonInput<MouseButton>>,
    selected_ships: Query<Entity, (With<EntityIsSelected>, With<Ship>)>,
    mouse_cursor: Res<MouseCursor>,
    mut event_writer: EventWriter<MoveShipToPositionCommand>,
) {
    if !input.just_released(MouseButton::Right) {
        return;
    }

    let Some(position) = &mouse_cursor.sector_space else {
        return;
    };

    event_writer.write_batch(
        selected_ships
            .iter()
            .map(|entity| MoveShipToPositionCommand {
                entity,
                position: position.sector_position,
            }),
    );
}

/// Tells a ship to move to a specific position.
/// TODO: Technically this could just be a generic StartManualTaskCommand<TaskData>... or even just a list of TaskKind enums?
/// TODO: Move somewhere higher, if this is supposed to be used by AI as well, AI doesn't need to depend on player controls
#[derive(Event)]
pub struct MoveShipToPositionCommand {
    pub entity: Entity,
    pub position: SectorPosition,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::event::EventIterator;
    use bevy::input::ButtonInput;
    use bevy::prelude::{Events, MouseButton, Vec2, World};
    use common::components::Ship;
    use common::session_data::ShipConfigId;
    use common::types::entity_wrappers::SectorEntity;
    use common::types::persistent_entity_id::PersistentShipId;
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
            .spawn((
                Ship::new(PersistentShipId::next(), ShipConfigId::from_name("asdf")),
                EntityIsSelected {},
            ))
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

        test_events::<MoveShipToPositionCommand, _>(&mut app, |mut events| {
            assert!(events.next().is_none())
        });

        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .release(MouseButton::Right);

        app.update();

        test_events::<MoveShipToPositionCommand, _>(&mut app, |mut events| {
            let event = events.next().unwrap();
            assert_eq!(event.entity, entity);
            assert_eq!(
                event.position.local_position,
                sector_position.local_position
            );
            assert_eq!(event.position.sector, sector_position.sector);
        });
    }
}
