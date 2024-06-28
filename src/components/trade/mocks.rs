use crate::components::inventory::InventoryElement;
use crate::components::{
    BuyOrderData, BuyOrders, OrderData, SellOrderData, SellOrders, TradeOrder,
};
use crate::constants;
use crate::game_data::ItemDefinition;
use crate::utils::PriceSetting;

impl BuyOrders {
    pub fn mock(items: Vec<&ItemDefinition>) -> Self {
        BuyOrders::from_vec(
            items
                .iter()
                .map(|item| {
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
                    (item.id, order)
                })
                .collect(),
        )
    }
}

impl SellOrders {
    pub fn mock(items: Vec<&ItemDefinition>) -> Self {
        SellOrders::from_vec(
            items
                .iter()
                .map(|item| {
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
                    (item.id, order)
                })
                .collect(),
        )
    }
}
