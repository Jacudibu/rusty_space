use crate::components::{BuyOrders, Inventory, SellOrders};
use bevy::prelude::Mut;

pub fn update_orders(
    inventory: &Inventory,
    buy_orders: Option<Mut<BuyOrders>>,
    sell_orders: Option<Mut<SellOrders>>,
) {
    if let Some(mut buy_orders) = buy_orders {
        buy_orders.update(inventory);
    }
    if let Some(mut sell_orders) = sell_orders {
        sell_orders.update(inventory);
    }
}
