use crate::persistence::PersistentStationId;
use crate::persistence::data::v1::inventory_save_data::InventorySaveData;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::simulation::prelude::SimulationTimestamp;
use crate::utils::PriceSetting;
use common::game_data::{
    ConstructableModuleId, ItemId, ProductionModuleId, RecipeId, ShipyardModuleId,
};
use common::session_data::ShipConfigId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ProductionSaveData {
    pub modules: Vec<ProductionModuleSaveData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ProductionModuleSaveData {
    pub module_id: ProductionModuleId,
    pub amount: u32,
    pub running_recipes: Vec<RunningProductionModuleQueueElementSaveData>,
    pub queued_recipes: Vec<ProductionModuleQueueElementSaveData>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ProductionModuleQueueElementSaveData {
    pub recipe: RecipeId,
    pub is_repeating: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RunningProductionModuleQueueElementSaveData {
    pub recipe: RecipeId,
    pub finished_at: SimulationTimestamp,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ShipyardModuleSaveData {
    pub module_id: ShipyardModuleId,
    pub amount: u32,
    pub active: Vec<ActiveShipyardOrderSaveData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ActiveShipyardOrderSaveData {
    pub finished_at: SimulationTimestamp,
    pub ship_config: ShipConfigId,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ShipyardSaveData {
    pub queue: Vec<ShipConfigId>,
    pub modules: Vec<ShipyardModuleSaveData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ConstructionSiteSaveData {
    pub queue: Vec<ConstructableModuleId>,
    pub current_progress: f32,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct StationSaveData {
    pub id: PersistentStationId,
    pub name: String,
    pub position: LocalHexPosition,
    pub inventory: InventorySaveData,
    pub production_modules: Option<ProductionSaveData>,
    pub shipyard_modules: Option<ShipyardSaveData>,
    pub buy_orders: Option<SerializedBuyOrder>,
    pub sell_orders: Option<SerializedSellOrder>,
    pub construction_site: Option<ConstructionSiteSaveData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SerializedBuyOrder {
    pub orders: Vec<SerializedBuyOrderData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SerializedSellOrder {
    pub orders: Vec<SerializedSellOrderData>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SerializedBuyOrderData {
    pub item_id: ItemId,
    pub amount: u32,

    pub buy_up_to: u32,
    pub price_setting: PriceSetting,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SerializedSellOrderData {
    pub item_id: ItemId,
    pub amount: u32,

    pub keep_at_least: u32,
    pub price_setting: PriceSetting,
}
