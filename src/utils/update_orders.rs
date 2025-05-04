use bevy::prelude::Mut;
use common::components::Inventory;
use common::components::SellOrders;
use common::components::{BuyOrders, TradeOrder};
use common::game_data::ItemManifest;

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
