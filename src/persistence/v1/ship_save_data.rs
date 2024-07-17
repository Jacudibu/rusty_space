use crate::components::{Inventory, Ship};
use crate::persistence::v1::inventory_save_data::InventorySaveData;
use crate::persistence::v1::task_save_data::TaskSaveData;
use crate::persistence::{AllEntityIdMaps, ComponentWithPersistentId, PersistentShipId};
use crate::physics::ShipVelocity;
use crate::ship_ai::TaskQueue;
use bevy::core::Name;
use bevy::math::{EulerRot, Vec2};
use bevy::prelude::Transform;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShipSaveData {
    pub id: PersistentShipId,
    pub name: String,
    pub position: Vec2,
    pub forward_velocity: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
    // TODO: behavior
    pub task_queue: Vec<TaskSaveData>,
    pub inventory: InventorySaveData,
}

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
