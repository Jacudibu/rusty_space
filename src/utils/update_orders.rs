use crate::components::Inventory;
use crate::components::SellOrders;
use crate::components::{BuyOrders, TradeOrder};
use crate::game_data::ItemManifest;
use bevy::prelude::Mut;

pub fn update_orders(
    inventory: &Inventory,
    buy_orders: Option<Mut<BuyOrders>>,
    sell_orders: Option<Mut<SellOrders>>,
    item_manifest: &ItemManifest,
) {
    if let Some(mut buy_orders) = buy_orders {
        buy_orders.update(inventory, item_manifest);
    }
    if let Some(mut sell_orders) = sell_orders {
        sell_orders.update(inventory, item_manifest);
    }
}
