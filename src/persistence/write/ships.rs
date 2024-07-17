use crate::components::{Inventory, Ship};
use crate::persistence::data::v1::*;
use crate::persistence::{AllEntityIdMaps, ComponentWithPersistentId};
use crate::physics::ShipVelocity;
use crate::ship_ai::TaskQueue;
use bevy::core::Name;
use bevy::math::EulerRot;
use bevy::prelude::Transform;

impl ShipSaveData {
    pub fn from(
        (ship, name, transform, task_queue, velocity, inventory): (
            &Ship,
            &Name,
            &Transform,
            &TaskQueue,
            &ShipVelocity,
            &Inventory,
        ),
        all_entity_id_maps: &AllEntityIdMaps,
    ) -> Self {
        Self {
            id: ship.id(),
            name: name.to_string(),
            position: transform.translation.truncate(),
            forward_velocity: velocity.forward,
            rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
            angular_velocity: velocity.angular,
            task_queue: task_queue
                .queue
                .iter()
                .map(|x| TaskSaveData::from(x, all_entity_id_maps))
                .collect(),
            inventory: InventorySaveData::from(inventory),
        }
    }
}
