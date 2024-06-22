use crate::components::Storage;
use crate::ids::ItemId;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct BuyOrders {
    pub orders: HashMap<ItemId, BuyOrderData>,
}

impl BuyOrders {
    pub fn mock_buying_item(item_id: ItemId) -> Self {
        let mut orders = HashMap::new();
        orders.insert(
            item_id,
            BuyOrderData {
                amount: u32::MAX,
                buy_up_to: u32::MAX,
                price: 10,
            },
        );

        Self { orders }
    }

    pub fn update(&mut self, storage: &Storage) {
        for (item_id, order) in &mut self.orders {
            order.update(storage.get_item_amount(item_id));
        }
    }
}

pub struct BuyOrderData {
    pub amount: u32,
    pub price: u32,

    pub buy_up_to: u32,
}

impl BuyOrderData {
    fn update(&mut self, stored_amount: u32) {
        if stored_amount > self.buy_up_to {
            self.amount = 0;
        } else {
            self.amount = self.buy_up_to - stored_amount;
        }
    }
}

#[derive(Component)]
pub struct SellOrders {
    pub orders: HashMap<ItemId, SellOrderData>,
}

impl SellOrders {
    pub fn mock_selling_item(item_id: ItemId) -> Self {
        let mut orders = HashMap::new();
        orders.insert(
            item_id,
            SellOrderData {
                amount: u32::MAX,
                keep_at_least: 0,
                price: 5,
            },
        );

        Self { orders }
    }

    pub fn update(&mut self, storage: &Storage) {
        for (item_id, order) in &mut self.orders {
            order.update(storage.get_item_amount(item_id));
        }
    }
}

pub struct SellOrderData {
    pub amount: u32,
    pub price: u32,

    pub keep_at_least: u32,
}

impl SellOrderData {
    fn update(&mut self, stored_amount: u32) {
        if stored_amount < self.keep_at_least {
            self.amount = 0;
        } else {
            self.amount = stored_amount - self.keep_at_least;
        }
    }
}
