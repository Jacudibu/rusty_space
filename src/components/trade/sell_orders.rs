use crate::components::inventory::InventoryElement;
use crate::components::{OrderData, TradeOrder};
use crate::game_data::ItemId;
use crate::utils::PriceSetting;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component, Default)]
pub struct SellOrders {
    orders: HashMap<ItemId, SellOrderData>,
}

pub struct SellOrderData {
    pub amount: u32,
    pub price: u32,

    pub keep_at_least: u32,
    pub price_setting: PriceSetting,
}

impl TradeOrder<SellOrderData> for SellOrders {
    fn orders(&self) -> &HashMap<ItemId, SellOrderData> {
        &self.orders
    }

    fn orders_mut(&mut self) -> &mut HashMap<ItemId, SellOrderData> {
        &mut self.orders
    }
}

impl OrderData for SellOrderData {
    fn update(&mut self, capacity: u32, inventory_element: Option<&InventoryElement>) {
        let stored_amount = if let Some(inventory_element) = inventory_element {
            inventory_element.current - inventory_element.planned_selling
        } else {
            0
        };

        if stored_amount < self.keep_at_least {
            self.amount = 0;
            self.price = self.price_setting.calculate_price(0, capacity) + 1;
        } else {
            self.amount = stored_amount - self.keep_at_least;
            self.price = self.price_setting.calculate_price(stored_amount, capacity);
        }
    }
}
