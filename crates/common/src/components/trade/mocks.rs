use crate::components::inventory::InventoryElement;
use crate::components::{
    BuyOrderData, BuyOrders, Inventory, OrderData, SellOrderData, SellOrders, TradeOrder,
};
use crate::constants;
use crate::enums::price_setting::PriceSetting;
use crate::game_data::{ItemData, ItemManifest};

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
                    order.update_price(
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
    pub fn mock(
        buys: &[&ItemData],
        sells: &Vec<&ItemData>,
        inventory: &mut Inventory,
        item_manifest: &ItemManifest,
    ) -> Self {
        let sharing_count = (buys.len() + sells.len()) as u32;
        SellOrders::from_vec(
            sells
                .iter()
                .map(|item| {
                    let capacity =
                        constants::MOCK_STATION_INVENTORY_SIZE / sharing_count / item.size;
                    inventory.set_purchase_reservation(&item.id, capacity, item_manifest);
                    let mut order = SellOrderData {
                        amount: capacity,
                        keep_at_least: 0,
                        price: 100,
                        price_setting: PriceSetting::Dynamic(item.price),
                    };
                    order.update_price(capacity, inventory.get(&item.id));
                    (item.id, order)
                })
                .collect(),
        )
    }
}
