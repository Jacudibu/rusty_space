use crate::components::InventoryComponent;
use crate::components::inventory::InventoryElement;
use crate::game_data::{ItemId, ItemManifest};
use bevy::prelude::Component;
use bevy::utils::HashMap;

pub trait TradeOrder<TOrderData: OrderData>: Default + Component {
    fn orders(&self) -> &HashMap<ItemId, TOrderData>;
    fn orders_mut(&mut self) -> &mut HashMap<ItemId, TOrderData>;

    /// Updates the prices for all orders given the current inventory situation.
    fn update(&mut self, inventory: &InventoryComponent, item_manifest: &ItemManifest) {
        for (item_id, order) in self.orders_mut() {
            order.update_price(
                inventory.remaining_space_for(item_id, item_manifest),
                inventory.get(item_id),
            );
        }
    }

    fn from_vec(vec: Vec<(ItemId, TOrderData)>) -> Self {
        let mut result = Self::default();
        let orders = result.orders_mut();

        for (item_id, order) in vec {
            orders.insert(item_id, order);
        }

        result
    }
}

pub trait OrderData {
    /// Updates the order amount and cached price
    fn update_price(&mut self, capacity: u32, inventory_element: Option<&InventoryElement>);
}
