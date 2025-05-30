use crate::components::inventory::InventoryElement;
use crate::components::{OrderData, TradeOrder};
use crate::game_data::ItemId;
use crate::types::price_setting::PriceSetting;
use bevy::platform::collections::HashMap;
use bevy::prelude::Component;

/// A component for any entity that actively looks to buy items.
#[derive(Component, Default)]
pub struct BuyOrders {
    pub orders: HashMap<ItemId, BuyOrderData>,
}

pub struct BuyOrderData {
    pub amount: u32,
    pub price: u32,

    pub buy_up_to: u32,
    pub price_setting: PriceSetting,
}

impl TradeOrder<BuyOrderData> for BuyOrders {
    fn orders(&self) -> &HashMap<ItemId, BuyOrderData> {
        &self.orders
    }

    fn orders_mut(&mut self) -> &mut HashMap<ItemId, BuyOrderData> {
        &mut self.orders
    }
}

impl OrderData for BuyOrderData {
    fn update_price(&mut self, _item_capacity: u32, inventory_element: Option<&InventoryElement>) {
        let stored_amount = if let Some(inventory_element) = inventory_element {
            inventory_element.current + inventory_element.planned_incoming
        } else {
            0
        };

        if stored_amount > self.buy_up_to {
            self.amount = 0;
            self.price = 0;
        } else {
            self.amount = self.buy_up_to - stored_amount;
            self.price = self
                .price_setting
                .calculate_price(stored_amount, self.buy_up_to);
        }
    }
}
