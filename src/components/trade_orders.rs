use crate::components::inventory::InventoryElement;
use crate::components::Inventory;
use crate::data::{ItemDefinition, ItemId};
use crate::utils::PriceRange;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct BuyOrders {
    pub orders: HashMap<ItemId, BuyOrderData>,
}

impl BuyOrders {
    pub fn mock_buying_item(item: &ItemDefinition) -> Self {
        let mut orders = HashMap::new();

        let mut order = BuyOrderData {
            amount: u32::MAX,
            buy_up_to: u32::MAX,
            price: 1,
            price_setting: PriceSetting::Dynamic(item.price),
        };
        order.update(Some(&InventoryElement {
            currently_available: 0,
            total: 0,
            ..Default::default()
        }));
        orders.insert(item.id, order);

        Self { orders }
    }

    pub fn update(&mut self, inventory: &Inventory) {
        for (item_id, order) in &mut self.orders {
            order.update(inventory.get(item_id));
        }
    }
}

pub struct BuyOrderData {
    pub amount: u32,
    pub price: u32,

    pub buy_up_to: u32,
    pub price_setting: PriceSetting,
}

impl BuyOrderData {
    /// Updates the order amount and cached price
    fn update(&mut self, inventory_element: Option<&InventoryElement>) {
        let (stored_amount, total) = if let Some(inventory_element) = inventory_element {
            (
                inventory_element.currently_available + inventory_element.planned_buying,
                inventory_element.total,
            )
        } else {
            (0, 0)
        };

        if stored_amount > self.buy_up_to {
            self.amount = 0;
            self.price = 0;
        } else {
            self.amount = self.buy_up_to - stored_amount;
            self.price = self.price_setting.calculate_price(total, self.buy_up_to);
        }
    }
}

#[derive(Component)]
pub struct SellOrders {
    pub orders: HashMap<ItemId, SellOrderData>,
}

impl SellOrders {
    pub fn mock_selling_item(item: &ItemDefinition) -> Self {
        let mut orders = HashMap::new();
        let mut order = SellOrderData {
            amount: u32::MAX,
            keep_at_least: 0,
            price: u32::MAX,
            price_setting: PriceSetting::Dynamic(item.price),
        };
        order.update(Some(&InventoryElement {
            currently_available: u32::MAX,
            total: u32::MAX,
            ..Default::default()
        }));
        orders.insert(item.id, order);

        Self { orders }
    }

    pub fn update(&mut self, inventory: &Inventory) {
        for (item_id, order) in &mut self.orders {
            order.update(inventory.get(item_id));
        }
    }
}

pub struct SellOrderData {
    pub amount: u32,
    pub price: u32,

    pub keep_at_least: u32,
    pub price_setting: PriceSetting,
}

impl SellOrderData {
    fn update(&mut self, inventory_element: Option<&InventoryElement>) {
        let (stored_amount, total) = if let Some(inventory_element) = inventory_element {
            (
                inventory_element.currently_available - inventory_element.planned_selling,
                inventory_element.total,
            )
        } else {
            (0, 0)
        };

        if stored_amount < self.keep_at_least {
            self.amount = 0;
            self.price = u32::MAX;
        } else {
            self.amount = stored_amount - self.keep_at_least;
            self.price = self
                .price_setting
                .calculate_price(total, self.keep_at_least);
        }
    }
}

pub enum PriceSetting {
    Fixed(u32),
    Dynamic(PriceRange),
}

impl PriceSetting {
    pub fn calculate_price(&self, storage: u32, capacity: u32) -> u32 {
        match self {
            PriceSetting::Fixed(value) => *value,
            PriceSetting::Dynamic(range) => range.calculate(storage as f32 / capacity as f32),
        }
    }
}
