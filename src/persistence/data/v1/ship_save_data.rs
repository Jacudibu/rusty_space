use crate::persistence::data::v1::inventory_save_data::InventorySaveData;
use crate::persistence::data::v1::task_save_data::TaskSaveData;
use crate::persistence::PersistentShipId;
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShipSaveData {
    pub id: PersistentShipId,
    pub name: String,
    pub position: Vec2, // TODO: use LocalHexPosition
    pub forward_velocity: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
    // TODO: behavior
    pub task_queue: Vec<TaskSaveData>,
    pub inventory: InventorySaveData,
}
