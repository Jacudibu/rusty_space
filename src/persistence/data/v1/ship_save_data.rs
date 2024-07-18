use crate::persistence::data::v1::inventory_save_data::InventorySaveData;
use crate::persistence::data::v1::task_save_data::TaskSaveData;
use crate::persistence::PersistentShipId;
use crate::ship_ai::AutoMineState;
use crate::universe_builder::LocalHexPosition;
use crate::utils::SimulationTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShipSaveData {
    pub id: PersistentShipId,
    pub name: String,
    pub position: LocalHexPosition,
    pub forward_velocity: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub behavior: ShipBehaviorSaveData,
    pub task_queue: Vec<TaskSaveData>,
    pub inventory: InventorySaveData,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum ShipBehaviorSaveData {
    AutoTrade {
        next_idle_update: SimulationTimestamp,
    },
    AutoMine {
        next_idle_update: SimulationTimestamp,
        state: AutoMineState,
    },
}
