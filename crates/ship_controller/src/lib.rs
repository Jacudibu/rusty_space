use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{Entity, Event, EventWriter, MouseButton, Plugin, Query, Res, Update, With};
use common::components::Ship;
use entity_selection::components::EntityIsSelected;

/// Adds ways through which the user can send command-events to control ships manually.
/// The same events may be used by AI.
pub struct ShipControllerPlugin;
impl Plugin for ShipControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveShipCommand>();
        app.add_systems(Update, send_move_command);
    }
}

/// This [System] sends a move command when the user right clicks into empty space.
// TODO: Actually add the position
pub(crate) fn send_move_command(
    input: Res<ButtonInput<MouseButton>>,
    selected_ships: Query<Entity, (With<EntityIsSelected>, With<Ship>)>,
    mut event_writer: EventWriter<MoveShipCommand>,
) {
    if !input.just_released(MouseButton::Right) {
        return;
    }

    event_writer.write_batch(
        selected_ships
            .iter()
            .map(|entity| MoveShipCommand { entity }),
    );
}

/// Tells a ship to move to a specific position.
/// TODO: Technically this could just be a generic StartManualTaskCommand<TaskData>... or even just a list of TaskKind enums?
/// TODO: Move somewhere higher, if this is supposed to be used by AI as well, AI doesn't need to depend on player controls
#[derive(Event)]
pub struct MoveShipCommand {
    pub entity: Entity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ButtonInput;
    use bevy::prelude::{Events, MouseButton};
    use common::components::Ship;
    use common::session_data::ShipConfigId;
    use common::types::persistent_entity_id::PersistentShipId;
    use entity_selection::components::EntityIsSelected;
    use test_utils::test_app::TestApp;

    fn test_app() -> TestApp {
        let mut app = TestApp::default();
        app.add_plugins(ShipControllerPlugin);
        app.insert_resource(ButtonInput::<MouseButton>::default());
        app
    }

    #[test]
    fn right_clicking_with_selected_ship_should_send_move_command() {
        let mut app = test_app().build();

        app.world_mut().spawn((
            Ship::new(PersistentShipId::next(), ShipConfigId::from_name("asdf")),
            EntityIsSelected {},
        ));

        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);

        app.update();

        let events = app.world().resource::<Events<MoveShipCommand>>();
        let mut event_reader = events.get_cursor();
        assert!(event_reader.read(events).next().is_none());

        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .release(MouseButton::Right);

        app.update();

        let events = app.world().resource::<Events<MoveShipCommand>>();
        let mut event_reader = events.get_cursor();
        let event = event_reader.read(events).next().unwrap();

        todo!("validate values in event")
    }
}
