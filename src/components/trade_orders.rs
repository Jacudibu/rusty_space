use crate::components::inventory::InventoryElement;
use crate::components::Inventory;
use crate::constants;
use crate::game_data::{ItemDefinition, ItemId};
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
            amount: constants::MOCK_INVENTORY_SIZE,
            buy_up_to: constants::MOCK_INVENTORY_SIZE,
            price: 1,
            price_setting: PriceSetting::Dynamic(item.price),
        };
        order.update(
            constants::MOCK_INVENTORY_SIZE,
            Some(&InventoryElement {
                currently_available: 0,
                total: 0,
                ..Default::default()
            }),
        );
        orders.insert(item.id, order);

        Self { orders }
    }

    pub fn update(&mut self, inventory: &Inventory) {
        for (item_id, order) in &mut self.orders {
            order.update(inventory.capacity, inventory.get(item_id));
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
    fn update(&mut self, capacity: u32, inventory_element: Option<&InventoryElement>) {
        let stored_amount = if let Some(inventory_element) = inventory_element {
            inventory_element.currently_available + inventory_element.planned_buying
        } else {
            0
        };

        if stored_amount > self.buy_up_to {
            self.amount = 0;
            self.price = 0;
        } else {
            self.amount = self.buy_up_to - stored_amount;
            self.price = self.price_setting.calculate_price(stored_amount, capacity);
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
            amount: constants::MOCK_INVENTORY_SIZE,
            keep_at_least: 0,
            price: 100,
            price_setting: PriceSetting::Dynamic(item.price),
        };
        order.update(
            constants::MOCK_INVENTORY_SIZE,
            Some(&InventoryElement {
                currently_available: constants::MOCK_INVENTORY_SIZE,
                total: constants::MOCK_INVENTORY_SIZE,
                ..Default::default()
            }),
        );
        orders.insert(item.id, order);

        Self { orders }
    }

    pub fn update(&mut self, inventory: &Inventory) {
        for (item_id, order) in &mut self.orders {
            order.update(inventory.capacity, inventory.get(item_id));
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
    fn update(&mut self, capacity: u32, inventory_element: Option<&InventoryElement>) {
        let stored_amount = if let Some(inventory_element) = inventory_element {
            inventory_element.currently_available - inventory_element.planned_selling
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
