use bevy::app::App;
use bevy::prelude::{Component, Entity};
use common::events::task_events::TaskStartedEvent;
use common::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;
use common::types::ship_tasks::{
    Construct, ExchangeWares, HarvestGas, MineAsteroid, Undock, UseGate,
};

pub mod test_app;

pub fn mock_entity_id<T: Component>(id: u32) -> TypedEntityWrapper<T> {
    Entity::from_raw(id).into()
}

// TODO: Probably not the best to maintain a list of these here...
//       Could be part of some "lite" test app in case this ever gets exclusively used by more tests?
pub fn add_all_event_writers_for_tests(app: &mut App) {
    app.add_event::<TaskStartedEvent<ExchangeWares>>();
    app.add_event::<TaskStartedEvent<UseGate>>();
    app.add_event::<TaskStartedEvent<Undock>>();
    app.add_event::<TaskStartedEvent<Construct>>();
    app.add_event::<TaskStartedEvent<MineAsteroid>>();
    app.add_event::<TaskStartedEvent<HarvestGas>>();
}
