use crate::simulation::prelude::TaskComponent;
use bevy::prelude::Component;

/// A ship with this component will be idle until it receives a Signal through an event.
#[derive(Component)]
pub struct AwaitingSignal {}
impl TaskComponent for AwaitingSignal {
    fn can_be_aborted() -> bool {
        true
    }
}
