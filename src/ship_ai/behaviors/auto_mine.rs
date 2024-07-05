use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform};

use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, TradeOrder};
use crate::gizmos::find_path;
use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::trade_plan::TradePlan;
use crate::utils::{ExchangeWareData, SimulationTime, SimulationTimestamp, TradeIntent};

#[derive(Eq, PartialEq)]
enum AutoMineState {
    Mining,
    Trading,
}

#[derive(Component)]
pub struct AutoMineBehavior {
    pub next_idle_update: SimulationTimestamp,
    state: AutoMineState,
}

impl Default for AutoMineBehavior {
    fn default() -> Self {
        Self {
            next_idle_update: SimulationTimestamp::MIN,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut AutoMineBehavior, &InSector), ShipIsIdleFilter>,
    mut buy_orders: Query<(Entity, &mut BuyOrders, &InSector)>,
    mut inventories: Query<&mut Inventory>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&Transform>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, ship_sector)| {
            behavior.next_idle_update.add_milliseconds(2000);
            let inventory = inventories.get_mut(ship_entity).unwrap();
            let used_inventory_space = inventory.used();

            if behavior.state == AutoMineState::Mining && used_inventory_space == inventory.capacity
            {
                behavior.state = AutoMineState::Trading;
            } else if behavior.state == AutoMineState::Trading && used_inventory_space == 0 {
                behavior.state = AutoMineState::Trading;
            }

            match behavior.state {
                AutoMineState::Mining => {

                    // TODO: Find closest asteroid, go ham on it
                }
                AutoMineState::Trading => {
                    // TODO: Sell all items in inventory
                }
            }
        });
}

fn update_buy_and_sell_orders_for_entity(
    entity: Entity,
    inventory: &Inventory,
    buy_orders: &mut Query<(Entity, &mut BuyOrders, &InSector)>,
    sell_orders: &mut Query<(Entity, &mut SellOrders, &InSector)>,
) {
    if let Ok(mut buy_orders) = buy_orders.get_mut(entity) {
        buy_orders.1.update(inventory);
    }
    if let Ok(mut sell_orders) = sell_orders.get_mut(entity) {
        sell_orders.1.update(inventory);
    }
}
