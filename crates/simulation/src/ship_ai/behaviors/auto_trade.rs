use bevy::prelude::{Entity, EventWriter, Query, Res};

use crate::ship_ai::task_filters::ShipIsIdleFilter;
use crate::ship_ai::trade_plan::TradePlan;
use common::components::ship_behavior::ShipBehavior;
use common::components::{BuyOrders, InSector, Inventory, SellOrders};
use common::constants;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskInsertionMode};
use common::game_data::ItemManifest;
use common::simulation_time::SimulationTime;
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_behaviors::AutoTradeBehavior;
use common::types::ship_tasks::ExchangeWares;

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut ShipBehavior<AutoTradeBehavior>), ShipIsIdleFilter>,
    buy_orders: Query<(Entity, &BuyOrders, &InSector)>,
    sell_orders: Query<(Entity, &SellOrders, &InSector)>,
    inventories: Query<&Inventory>,
    item_manifest: Res<ItemManifest>,
    mut event_writer: EventWriter<InsertTaskIntoQueueCommand<ExchangeWares>>,
) {
    let now = simulation_time.now();

    // TODO: We should collect the best deals for each system in between simulation ticks and then just work on that list
    //       Maintaining it between ticks might be inefficient since production changes will shift everything around
    //       ...until then, we can only handle one ship per simulation run, now that orders are manipulated through events! Oops.

    let Some((ship_entity, mut behavior)) = ships
        .iter_mut()
        .find(|(_, behavior)| now.has_passed(behavior.next_idle_update))
    else {
        return;
    };
    let inventory = inventories.get(ship_entity).unwrap();
    let plan =
        TradePlan::search_for_trade_run(inventory, &buy_orders, &sell_orders, &item_manifest);
    let Some(plan) = plan else {
        behavior.next_idle_update =
            now.add_seconds(constants::SECONDS_BETWEEN_SHIP_BEHAVIOR_IDLE_UPDATES);
        return;
    };

    let purchase = ExchangeWares::new(
        plan.seller,
        ExchangeWareData::Buy(plan.item_id, plan.amount),
    );

    let sale = ExchangeWares::new(
        plan.buyer,
        ExchangeWareData::Sell(plan.item_id, plan.amount),
    );

    // This depends on events being read synchronously in sequence. Let's hope that never changes?
    event_writer.write(InsertTaskIntoQueueCommand {
        entity: ship_entity,
        task_data: purchase,
        insertion_mode: TaskInsertionMode::Append,
    });
    event_writer.write(InsertTaskIntoQueueCommand {
        entity: ship_entity,
        task_data: sale,
        insertion_mode: TaskInsertionMode::Append,
    });
}
