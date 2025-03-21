use crate::components::{
    BuyOrderData, BuyOrders, InSector, Inventory, SectorComponent, SellOrderData, SellOrders,
    StationComponent, TradeOrder,
};
use crate::game_data::{ItemId, ProductionModuleId, ShipyardModuleId};
use crate::persistence::ComponentWithPersistentId;
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::simulation::production::{
    OngoingShipConstructionOrder, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use bevy::core::Name;
use bevy::prelude::Query;

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

impl ProductionModuleSaveData {
    pub fn from((id, module): (&ProductionModuleId, &ProductionModule)) -> Self {
        Self {
            module_id: *id,
            amount: module.amount,
            queued_recipes: todo!(),
            #[allow(unreachable_code)]
            running_recipes: todo!(),
        }
    }
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
            amount: module.amount,
            active: module
                .active
                .iter()
                .map(ActiveShipyardOrderSaveData::from)
                .collect(),
        }
    }
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

impl SerializedBuyOrderData {
    pub fn from((id, data): (&ItemId, &BuyOrderData)) -> Self {
        Self {
            item_id: *id,
            amount: data.amount,
            price_setting: data.price_setting,
            buy_up_to: data.buy_up_to,
        }
    }
}

impl SerializedSellOrderData {
    pub fn from((id, data): (&ItemId, &SellOrderData)) -> Self {
        Self {
            item_id: *id,
            amount: data.amount,
            price_setting: data.price_setting,
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
            &StationComponent,
            &Name,
            &InSector,
            &SimulationTransform,
            &Inventory,
            Option<&ProductionComponent>,
            Option<&ShipyardComponent>,
            Option<&BuyOrders>,
            Option<&SellOrders>,
        ),
        sectors: &Query<&SectorComponent>,
    ) -> Self {
        Self {
            id: station.id(),
            name: name.to_string(),
            position: LocalHexPosition::from_in_sector(in_sector, &transform, sectors),
            inventory: InventorySaveData::from(inventory),
            buy_orders: buy_orders.map(SerializedBuyOrder::from),
            sell_orders: sell_orders.map(SerializedSellOrder::from),
            production_modules: production.map(ProductionSaveData::from),
            shipyard_modules: shipyard.map(ShipyardSaveData::from),
            construction_site: todo!(),
        }
    }
}
