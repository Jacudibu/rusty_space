use crate::game_data::ItemId;
use crate::persistence::data::v1::inventory_save_data::InventorySaveData;
use crate::persistence::data::v1::task_save_data::TaskSaveData;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::PersistentShipId;
use crate::session_data::ShipConfigId;
use crate::simulation::prelude::SimulationTimestamp;
use crate::simulation::ship_ai::AutoMineState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct ShipSaveData {
    pub id: PersistentShipId,
    pub config_id: ShipConfigId,
    pub name: String,
    pub position: LocalHexPosition,
    pub forward_velocity: f32,
    pub rotation_degrees: f32,
    pub angular_velocity: f32,
    pub behavior: ShipBehaviorSaveData,
    pub task_queue: Vec<TaskSaveData>,
    pub inventory: InventorySaveData,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum ShipBehaviorSaveData {
    AutoTrade {
        next_idle_update: SimulationTimestamp,
    },
    AutoMine {
        next_idle_update: SimulationTimestamp,
        mined_ore: ItemId,
        state: AutoMineState,
    },
    AutoHarvest {
        next_idle_update: SimulationTimestamp,
        harvested_gas: ItemId,
        state: AutoMineState,
    },
}
