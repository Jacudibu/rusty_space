use crate::simulation::prelude::TaskComponent;
use crate::utils::TypedEntity;
use bevy::prelude::Component;

/// A ship with this component will be idle until it receives a Signal through an event.
#[derive(Component)]
#[component(immutable)]
pub struct AwaitingSignal {
    pub(crate) from: TypedEntity,
}

impl TaskComponent for AwaitingSignal {
    fn can_be_aborted() -> bool {
        true
    }
}
