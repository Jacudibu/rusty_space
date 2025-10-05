use bevy::app::App;
use bevy::ecs::message::MessageIterator;
use bevy::prelude::{Component, Entity, Message, Messages};
use common::events::task_events::TaskStartedEvent;
use common::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, Undock, UseGate,
};

pub mod test_app;

pub fn mock_entity_id<T: Component>(id: u32) -> TypedEntityWrapper<T> {
    Entity::from_raw_u32(id).unwrap().into()
}

// TODO: Probably not the best to maintain a list of these here...
//       Could be part of some "lite" test app in case this ever gets exclusively used by more tests?
pub fn add_all_event_writers_for_tests(app: &mut App) {
    app.add_message::<TaskStartedEvent<AwaitingSignal>>();
    app.add_message::<TaskStartedEvent<Construct>>();
    app.add_message::<TaskStartedEvent<DockAtEntity>>();
    app.add_message::<TaskStartedEvent<ExchangeWares>>();
    app.add_message::<TaskStartedEvent<HarvestGas>>();
    app.add_message::<TaskStartedEvent<MineAsteroid>>();
    app.add_message::<TaskStartedEvent<MoveToPosition>>();
    app.add_message::<TaskStartedEvent<MoveToSector>>();
    app.add_message::<TaskStartedEvent<MoveToEntity>>();
    app.add_message::<TaskStartedEvent<RequestAccess>>();
    app.add_message::<TaskStartedEvent<UseGate>>();
    app.add_message::<TaskStartedEvent<Undock>>();
}

/// Executes a function on a EventIterator for the provided Event.
/// I wish there was a way to just return the MessageReader, but that doesn't work.
pub fn test_events<T: Message, F: Fn(MessageIterator<T>)>(app: &mut App, event_tests: F) {
    let events = app.world().resource::<Messages<T>>();
    let mut event_reader = events.get_cursor();
    let events = event_reader.read(events);

    event_tests(events);
}
