use bevy::app::App;
use bevy::ecs::event::EventIterator;
use bevy::prelude::{Component, Entity, Event, Events};
use common::events::task_events::TaskStartedEvent;
use common::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, Undock, UseGate,
};

pub mod test_app;

pub fn mock_entity_id<T: Component>(id: u32) -> TypedEntityWrapper<T> {
    Entity::from_raw(id).into()
}

// TODO: Probably not the best to maintain a list of these here...
//       Could be part of some "lite" test app in case this ever gets exclusively used by more tests?
pub fn add_all_event_writers_for_tests(app: &mut App) {
    app.add_event::<TaskStartedEvent<AwaitingSignal>>();
    app.add_event::<TaskStartedEvent<Construct>>();
    app.add_event::<TaskStartedEvent<DockAtEntity>>();
    app.add_event::<TaskStartedEvent<ExchangeWares>>();
    app.add_event::<TaskStartedEvent<HarvestGas>>();
    app.add_event::<TaskStartedEvent<MineAsteroid>>();
    app.add_event::<TaskStartedEvent<MoveToPosition>>();
    app.add_event::<TaskStartedEvent<MoveToSector>>();
    app.add_event::<TaskStartedEvent<MoveToEntity>>();
    app.add_event::<TaskStartedEvent<RequestAccess>>();
    app.add_event::<TaskStartedEvent<UseGate>>();
    app.add_event::<TaskStartedEvent<Undock>>();
}

/// Executes a function on a EventIterator for the provided Event.
/// I wish there was a way to just return the EventReader, but that doesn't work.
pub fn test_events<T: Event, F: Fn(EventIterator<T>)>(app: &mut App, event_tests: F) {
    let events = app.world().resource::<Events<T>>();
    let mut event_reader = events.get_cursor();
    let events = event_reader.read(events);

    event_tests(events);
}
