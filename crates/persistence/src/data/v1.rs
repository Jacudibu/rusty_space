use bevy::prelude::{Resource, Vec2};
use common::game_data::{
    AsteroidDataId, ConstructableModuleId, ItemId, ProductionModuleId, RecipeId, ShipyardModuleId,
};
use common::session_data::ShipConfigId;
use common::simulation_time::SimulationTimestamp;
use common::types::celestial_mass::CelestialMass;
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::{
    PersistentAsteroidId, PersistentCelestialId, PersistentEntityId, PersistentGateId,
    PersistentShipId, PersistentStationId,
};
use common::types::price_setting::PriceSetting;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UniverseSaveData {
    pub gate_pairs: Vec<GatePairSaveData>,
    pub sectors: Vec<SectorSaveData>,
    pub ships: Vec<ShipSaveData>,
    pub stations: Vec<StationSaveData>,
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SaveDataCollection<T> {
    pub data: Vec<T>,
}

impl<T> Default for SaveDataCollection<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GatePairSaveData {
    pub from_id: PersistentGateId,
    pub from_position: LocalHexPosition,
    pub to_id: PersistentGateId,
    pub to_position: LocalHexPosition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InventorySaveData {
    pub items: Vec<(ItemId, u32)>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AsteroidSaveData {
    pub id: PersistentAsteroidId,
    pub manifest_id: AsteroidDataId,
    pub ore_item_id: ItemId,
    pub ore_current: u32,
    pub ore_max: u32,
    pub position: Vec2,
    pub rotation_degrees: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub lifetime: SimulationTimestamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AsteroidRespawnSaveData {
    pub id: PersistentAsteroidId,
    pub ore_max: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub timestamp: SimulationTimestamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SectorAsteroidSaveData {
    pub average_velocity: Vec2,
    pub asteroid_materials: Vec<ItemId>,
    pub live_asteroids: Vec<AsteroidSaveData>,
    pub respawning_asteroids: Vec<AsteroidRespawnSaveData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IndividualSectorCelestialSaveData {
    pub id: PersistentCelestialId,
    pub kind: CelestialKindSaveData,
    pub name: String,
    pub mass: CelestialMass,
    pub local_position: Vec2,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CelestialKindSaveData {
    Star,
    Terrestrial,
    GasGiant { resources: Vec<ItemId> },
}

impl Display for CelestialKindSaveData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CelestialKindSaveData::Star => f.write_str("Star"),
            CelestialKindSaveData::Terrestrial => f.write_str("Planet"),
            CelestialKindSaveData::GasGiant { .. } => f.write_str("Gas Giant"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SectorCelestialsSaveData {
    pub center_mass: CelestialMass,
    pub celestials: Vec<IndividualSectorCelestialSaveData>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct SectorFeatureSaveData {
    pub asteroids: Option<SectorAsteroidSaveData>,
    pub celestials: Option<SectorCelestialsSaveData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub features: SectorFeatureSaveData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum AutoMineStateSaveData {
    Mining,
    Trading,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum ShipBehaviorSaveData {
    AutoTrade,
    AutoConstruct,
    AutoMine {
        mined_ore: ItemId,
        state: AutoMineStateSaveData,
    },
    AutoHarvest {
        harvested_gas: ItemId,
        state: AutoMineStateSaveData,
    },
    HoldPosition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProductionSaveData {
    pub modules: Vec<ProductionModuleSaveData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProductionModuleSaveData {
    pub module_id: ProductionModuleId,
    pub amount: u32,
    pub running_recipes: Vec<RunningProductionModuleQueueElementSaveData>,
    pub queued_recipes: Vec<ProductionModuleQueueElementSaveData>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct ProductionModuleQueueElementSaveData {
    pub recipe: RecipeId,
    pub is_repeating: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct RunningProductionModuleQueueElementSaveData {
    pub recipe: RecipeId,
    pub finished_at: SimulationTimestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShipyardModuleSaveData {
    pub module_id: ShipyardModuleId,
    pub amount: u32,
    pub active: Vec<ActiveShipyardOrderSaveData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ActiveShipyardOrderSaveData {
    pub finished_at: SimulationTimestamp,
    pub ship_config: ShipConfigId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShipyardSaveData {
    pub queue: Vec<ShipConfigId>,
    pub modules: Vec<ShipyardModuleSaveData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConstructionSiteSaveData {
    pub queue: Vec<ConstructableModuleId>,
    pub current_progress: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerializedBuyOrder {
    pub orders: Vec<SerializedBuyOrderData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerializedSellOrder {
    pub orders: Vec<SerializedSellOrderData>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct SerializedBuyOrderData {
    pub item_id: ItemId,
    pub amount: u32,

    pub buy_up_to: u32,
    pub price_setting: PriceSetting,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct SerializedSellOrderData {
    pub item_id: ItemId,
    pub amount: u32,

    pub keep_at_least: u32,
    pub price_setting: PriceSetting,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskSaveData {
    ExchangeWares {
        target: PersistentEntityId,
        data: ExchangeWareSaveData,
    },
    MoveToEntity {
        target: PersistentEntityId,
        stop_at_target: bool,
        distance_to_target: f32,
    },
    UseGate {
        enter_gate: PersistentGateId,
        exit_sector: Hex,
    },
    MineAsteroid {
        target: PersistentAsteroidId,
        reserved: u32,
    },
    HarvestGas {
        target: PersistentCelestialId,
        gas: ItemId,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExchangeWareSaveData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}
