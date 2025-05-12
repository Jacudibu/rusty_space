use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use bevy::prelude::{Entity, Query};
use common::components::task_queue::TaskInsideQueue;
use common::components::task_queue::TaskQueue;
use common::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use common::constants;
use common::game_data::{ItemId, ItemManifest};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::{SectorEntity, TypedEntity};
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_tasks;

pub struct TradePlan {
    pub item_id: ItemId,
    pub amount: u32,
    pub profit: u32,
    pub seller: TypedEntity,
    pub seller_sector: SectorEntity,
    pub buyer: TypedEntity,
    pub buyer_sector: SectorEntity,
}

impl TradePlan {
    #[must_use]
    pub fn search_for_trade_run(
        inventory: &Inventory,
        buy_orders: &Query<(Entity, &mut BuyOrders, &InSector)>,
        sell_orders: &Query<(Entity, &mut SellOrders, &InSector)>,
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

    pub fn create_tasks_for_purchase(
        &self,
        all_sectors: &Query<&Sector>,
        all_transforms: &Query<&SimulationTransform>,
        ship_entity: Entity,
        ship_sector: &InSector,
        queue: &mut TaskQueue,
    ) {
        if ship_sector != self.seller_sector {
            let ship_pos = all_transforms.get(ship_entity).unwrap().translation;
            let path = pathfinding::find_path(
                all_sectors,
                all_transforms,
                ship_sector.get(),
                ship_pos,
                self.seller_sector,
                Some(all_transforms.get(self.seller.into()).unwrap().translation),
            )
            .unwrap();

            create_tasks_to_follow_path(queue, path);
        }

        queue.push_back(TaskInsideQueue::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: self.seller,
                stop_at_target: true,
                desired_distance_to_target: constants::DOCKING_DISTANCE_TO_STATION,
            },
        });
        queue.push_back(TaskInsideQueue::RequestAccess {
            data: ship_tasks::RequestAccess {
                target: self.seller,
            },
        });
        queue.push_back(TaskInsideQueue::DockAtEntity {
            data: ship_tasks::DockAtEntity {
                target: self.seller,
            },
        });
        queue.push_back(TaskInsideQueue::ExchangeWares {
            data: ship_tasks::ExchangeWares::new(
                self.seller,
                ExchangeWareData::Buy(self.item_id, self.amount),
            ),
        });
        queue.push_back(TaskInsideQueue::Undock {
            data: ship_tasks::Undock::default(),
        }) // TODO: Ideally that should be added dynamically at the start of MoveToEntity if we are docked
    }

    pub fn create_tasks_for_sale(
        &self,
        all_sectors: &Query<&Sector>,
        all_transforms: &Query<&SimulationTransform>,
        queue: &mut TaskQueue,
    ) {
        if self.seller_sector != self.buyer_sector {
            let seller_pos = all_transforms.get(self.seller.into()).unwrap().translation;
            let path = pathfinding::find_path(
                all_sectors,
                all_transforms,
                self.seller_sector,
                seller_pos,
                self.buyer_sector,
                Some(all_transforms.get(self.buyer.into()).unwrap().translation),
            )
            .unwrap();

            create_tasks_to_follow_path(queue, path);
        }

        queue.push_back(TaskInsideQueue::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: self.buyer,
                stop_at_target: true,
                desired_distance_to_target: constants::DOCKING_DISTANCE_TO_STATION,
            },
        });
        queue.push_back(TaskInsideQueue::RequestAccess {
            data: ship_tasks::RequestAccess { target: self.buyer },
        });
        queue.push_back(TaskInsideQueue::DockAtEntity {
            data: ship_tasks::DockAtEntity { target: self.buyer },
        });
        queue.push_back(TaskInsideQueue::ExchangeWares {
            data: ship_tasks::ExchangeWares::new(
                self.buyer,
                ExchangeWareData::Sell(self.item_id, self.amount),
            ),
        });
        queue.push_back(TaskInsideQueue::Undock {
            data: ship_tasks::Undock::default(),
        }) // TODO: Ideally that should be added dynamically at the start of MoveToEntity if we are docked
    }
}
