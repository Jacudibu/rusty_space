use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use bevy::prelude::{Entity, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use common::constants;
use common::game_data::{ItemId, ItemManifest};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::{SectorEntity, TypedEntity};
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_tasks;

/// Describes a complete trade run - first we buy cheap, then we sell high!
pub struct TradePlan {
    /// The [ItemId] of the item that's being traded.
    pub item_id: ItemId,
    /// The amount we are trading.
    pub amount: u32,
    /// The expected profit from this trade plan.
    pub profit: u32,
    /// The entity that's going to sell us their good.
    pub seller: TypedEntity,
    /// The sector in which the seller resides.
    pub seller_sector: SectorEntity,
    /// The entity that's going to buy our goods.
    pub buyer: TypedEntity,
    /// The sector in which our buyer resides.
    pub buyer_sector: SectorEntity,
}

impl TradePlan {
    #[must_use]
    pub fn search_for_trade_run(
        inventory: &Inventory,
        buy_orders: &Query<(Entity, &BuyOrders, &InSector)>,
        sell_orders: &Query<(Entity, &SellOrders, &InSector)>,
        item_manifest: &ItemManifest,
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

                        let amount = inventory
                            .remaining_space_for(item_id, item_manifest)
                            .min(buy_order.amount)
                            .min(sell_order.amount);
                        if amount == 0 {
                            // TODO: Add custom defined minimum amount so the player has an option to tell ships to not ferry around 1 item
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
                                seller: TypedEntity::AnyWithInventory(seller),
                                seller_sector: seller_sector.get(),
                                buyer: TypedEntity::AnyWithInventory(buyer),
                                buyer_sector: buyer_sector.get(),
                            });
                        }
                    }
                }
            }
        }

        best_offer
    }

    pub fn sell_anything_from_inventory(
        seller: Entity,
        seller_sector: &InSector,
        inventory: &Inventory,
        buy_orders: &Query<(Entity, &mut BuyOrders, &InSector)>,
    ) -> Option<Self> {
        let mut best_offer: Option<TradePlan> = None;

        for (buyer, buy_orders, buyer_sector) in buy_orders.iter() {
            if seller == buyer {
                continue;
            }

            for (item_id, inventory_entry) in inventory.inventory() {
                if let Some(buy_order) = buy_orders.orders().get(item_id) {
                    let amount = inventory_entry.total.min(buy_order.amount);
                    if amount == 0 {
                        continue;
                    }

                    let profit = buy_order.price * amount;

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
                            seller: TypedEntity::AnyWithInventory(seller),
                            seller_sector: seller_sector.get(),
                            buyer: TypedEntity::AnyWithInventory(buyer),
                            buyer_sector: buyer_sector.get(),
                        });
                    }
                }
            }
        }

        best_offer
    }
}
