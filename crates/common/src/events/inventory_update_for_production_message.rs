use bevy::prelude::{Entity, Message};

/// This event should be sent whenever an entity's inventory is being updated outside the production manager
///
/// More performant than querying with Changed<Inventory> since bevy won't need to iterate
/// through all entities matching the query every frame, plus it won't trigger itself recursively
/// ...the only risk is that we may forget to send it on inventory changes. What could go wrong?
#[derive(Message)]
pub struct InventoryUpdateForProductionMessage {
    pub entity: Entity,
}

impl InventoryUpdateForProductionMessage {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
