use crate::components::inventory::InventoryElement;
use crate::components::{
    BuyOrderData, BuyOrders, OrderData, SellOrderData, SellOrders, TradeOrder,
};
use crate::constants;
use crate::game_data::ItemData;
use crate::utils::PriceSetting;

impl BuyOrders {
    pub fn mock(buys: &Vec<&ItemData>, sells: &[&ItemData]) -> Self {
        let sharing_count = (buys.len() + sells.len()) as u32;
        BuyOrders::from_vec(
            buys.iter()
                .map(|item| {
                    let capacity =
                        constants::MOCK_STATION_INVENTORY_SIZE / sharing_count / item.size;
                    let mut order = BuyOrderData {
                        amount: capacity,
                        buy_up_to: capacity,
                        price: 1,
                        price_setting: PriceSetting::Dynamic(item.price),
                    };
                    order.update(
                        capacity,
                        Some(&InventoryElement {
                            current: 0,
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
    pub fn mock(buys: &[&ItemData], sells: &Vec<&ItemData>) -> Self {
        let sharing_count = (buys.len() + sells.len()) as u32;
        SellOrders::from_vec(
            sells
                .iter()
                .map(|item| {
                    let capacity =
                        constants::MOCK_STATION_INVENTORY_SIZE / sharing_count / item.size;
                    let mut order = SellOrderData {
                        amount: capacity,
                        keep_at_least: 0,
                        price: 100,
                        price_setting: PriceSetting::Dynamic(item.price),
                    };
                    order.update(
                        capacity,
                        Some(&InventoryElement {
                            current: capacity,
                            total: capacity,
                            ..Default::default()
                        }),
                    );
                    (item.id, order)
                })
                .collect(),
        )
    }
}
