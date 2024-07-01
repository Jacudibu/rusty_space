use crate::components::{BuyOrders, SellOrders, TradeOrder};
use crate::game_data::ItemId;
use crate::sectors::InSector;
use bevy::prelude::{Entity, Query};
use hexx::Hex;

pub struct TradePlan {
    pub item_id: ItemId,
    pub amount: u32,
    pub profit: u32,
    pub seller: Entity,
    pub seller_sector: Hex,
    pub buyer: Entity,
    pub buyer_sector: Hex,
}

impl TradePlan {
    pub fn create_from(
        storage_capacity: u32,
        buy_orders: &Query<(Entity, &mut BuyOrders, &InSector)>,
        sell_orders: &Query<(Entity, &mut SellOrders, &InSector)>,
    ) -> Option<Self> {
        let mut best_offer: Option<TradePlan> = None;

        for (buyer, buy_orders, buyer_sector) in buy_orders.iter() {
            for (seller, sell_orders, seller_sector) in sell_orders.iter() {
                if buyer == seller {
                    continue;
                }

                for (item_id, buy_order) in buy_orders.orders() {
                    if let Some(sell_order) = sell_orders.orders().get(item_id) {
                        if sell_order.price >= buy_order.price {
                            continue;
                        }

                        let amount = storage_capacity.min(buy_order.amount.min(sell_order.amount));
                        if amount == 0 {
                            // TODO: Add custom definable minimum amount
                            continue;
                        }

                        let profit = (buy_order.price - sell_order.price) * amount;

                        let is_this_a_better_offer = if let Some(existing_offer) = &best_offer {
                            profit > existing_offer.profit
                        } else {
                            true
                        };

                        if is_this_a_better_offer {
                            best_offer = Some(TradePlan {
                                item_id: *item_id,
                                amount,
                                profit,
                                seller,
                                seller_sector: seller_sector.sector,
                                buyer,
                                buyer_sector: buyer_sector.sector,
                            });
                        }
                    }
                }
            }
        }

        best_offer
    }
}
