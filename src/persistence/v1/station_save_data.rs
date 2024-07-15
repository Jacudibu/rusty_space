use crate::components::{
    BuyOrderData, BuyOrders, InSector, Inventory, SellOrderData, SellOrders, Station, TradeOrder,
};
use crate::game_data::{ItemId, ProductionModuleId, RecipeId, ShipyardModuleId};
use crate::persistence::v1::inventory_save_data::InventorySaveData;
use crate::persistence::AllEntityIdMaps;
use crate::production::{
    OngoingShipConstructionOrder, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::session_data::ShipConfigId;
use crate::utils::{PriceSetting, SimulationTimestamp};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::Transform;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductionSaveData {
    pub modules: Vec<ProductionModuleSaveData>,
}

impl ProductionSaveData {
    pub fn from(production: &ProductionComponent) -> Self {
        Self {
            modules: production
                .modules
                .iter()
                .map(ProductionModuleSaveData::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductionModuleSaveData {
    pub module_id: ProductionModuleId,
    pub amount: u32,
    pub recipe: RecipeId,
    pub finished_at: Option<SimulationTimestamp>,
}

impl ProductionModuleSaveData {
    pub fn from((id, module): (&ProductionModuleId, &ProductionModule)) -> Self {
        Self {
            module_id: *id,
            amount: module.amount,
            recipe: module.recipe,
            finished_at: module.current_run_finished_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShipyardModuleSaveData {
    pub module_id: ShipyardModuleId,
    pub active: Vec<ActiveShipyardOrderSaveData>,
}

#[derive(Serialize, Deserialize)]
pub struct ActiveShipyardOrderSaveData {
    pub finished_at: SimulationTimestamp,
    pub ship_config: ShipConfigId,
}

impl ActiveShipyardOrderSaveData {
    pub fn from(order: &OngoingShipConstructionOrder) -> Self {
        Self {
            ship_config: order.ship_config,
            finished_at: order.finished_at,
        }
    }
}

impl ShipyardModuleSaveData {
    pub fn from((id, module): (&ShipyardModuleId, &ShipyardModule)) -> Self {
        Self {
            module_id: *id,
            active: module
                .active
                .iter()
                .map(ActiveShipyardOrderSaveData::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShipyardSaveData {
    queue: Vec<ShipConfigId>,
    modules: Vec<ShipyardModuleSaveData>,
}

impl ShipyardSaveData {
    pub fn from(shipyard: &ShipyardComponent) -> Self {
        Self {
            queue: shipyard.queue.clone(),
            modules: shipyard
                .modules
                .iter()
                .map(ShipyardModuleSaveData::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StationSaveData {
    pub name: String,
    pub sector: Hex,
    pub position: Vec2,
    pub inventory: InventorySaveData,
    pub production_modules: Option<ProductionSaveData>,
    pub shipyard_modules: Option<ShipyardSaveData>,
    pub buy_orders: Option<SerializedBuyOrder>,
    pub sell_orders: Option<SerializedSellOrder>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBuyOrder {
    pub orders: Vec<SerializedBuyOrderData>,
}

impl SerializedBuyOrder {
    pub fn from(orders: &BuyOrders) -> Self {
        Self {
            orders: orders
                .orders()
                .iter()
                .map(SerializedBuyOrderData::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedSellOrder {
    pub orders: Vec<SerializedSellOrderData>,
}

impl SerializedSellOrder {
    pub fn from(orders: &SellOrders) -> Self {
        Self {
            orders: orders
                .orders()
                .iter()
                .map(SerializedSellOrderData::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBuyOrderData {
    pub item_id: ItemId,
    pub amount: u32,
    pub price: u32,

    pub buy_up_to: u32,
    pub price_setting: PriceSetting,
}

impl SerializedBuyOrderData {
    pub fn from((id, data): (&ItemId, &BuyOrderData)) -> Self {
        Self {
            item_id: *id,
            price: data.price,
            amount: data.amount,
            price_setting: data.price_setting.clone(),
            buy_up_to: data.buy_up_to,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedSellOrderData {
    pub item_id: ItemId,
    pub amount: u32,
    pub price: u32,

    pub keep_at_least: u32,
    pub price_setting: PriceSetting,
}

impl SerializedSellOrderData {
    pub fn from((id, data): (&ItemId, &SellOrderData)) -> Self {
        Self {
            item_id: *id,
            price: data.price,
            amount: data.amount,
            price_setting: data.price_setting.clone(),
            keep_at_least: data.keep_at_least,
        }
    }
}

impl StationSaveData {
    #[allow(clippy::type_complexity)]
    pub fn from(
        (
            station,
            name,
            in_sector,
            transform,
            inventory,
            production,
            shipyard,
            buy_orders,
            sell_orders,
        ): (
            &Station,
            &Name,
            &InSector,
            &Transform,
            &Inventory,
            Option<&ProductionComponent>,
            Option<&ShipyardComponent>,
            Option<&BuyOrders>,
            Option<&SellOrders>,
        ),
        all_entity_id_maps: &AllEntityIdMaps,
    ) -> Self {
        Self {
            name: name.to_string(),
            sector: all_entity_id_maps.sectors.entity_to_id()[&in_sector.sector],
            position: transform.translation.truncate(),
            inventory: InventorySaveData::from(inventory),
            buy_orders: buy_orders.map(SerializedBuyOrder::from),
            sell_orders: sell_orders.map(SerializedSellOrder::from),
            production_modules: production.map(ProductionSaveData::from),
            shipyard_modules: shipyard.map(ShipyardSaveData::from),
        }
    }
}
