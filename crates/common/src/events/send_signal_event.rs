use crate::types::entity_wrappers::ShipEntity;
use bevy::prelude::Event;

/// Sends a signal to an Entity. If the receiving Entity has an active [ShipTask]<[AwaitingSignal]>, that task will be completed.
#[derive(Event)]
pub struct SendSignalEvent {
    /// The entity which should receive the signal
    pub entity: ShipEntity,
}
