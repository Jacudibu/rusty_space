use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_cancellation_active::TaskCancellationForActiveTaskHandler;
use crate::ship_ai::task_cancellation_in_queue::TaskCancellationForTaskInQueueHandler;
use crate::ship_ai::task_creation::{
    GeneralPathfindingArgs, TaskCreationError, TaskCreationErrorReason, TaskCreationHandler,
    create_preconditions_and_dock_at_entity,
};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use crate::ship_ai::{NoArgs, TaskComponent};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventReader, EventWriter, Query, Res, error};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{BuyOrders, Inventory, SellOrders, TradeOrder};
use common::events::inventory_update_for_production_event::InventoryUpdateForProductionEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileInQueueEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTime};
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_tasks::ExchangeWares;
use common::types::trade_intent::TradeIntent;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<ExchangeWares> {
    fn can_be_cancelled_while_active() -> bool {
        false
    }
}

impl ShipTask<ExchangeWares> {
    fn run(&self, now: CurrentSimulationTimestamp) -> TaskResult {
        if now.has_not_passed(self.finishes_at) {
            TaskResult::Ongoing
        } else {
            TaskResult::Finished
        }
    }

    fn complete(
        &self,
        this_entity: Entity,
        all_storages: &mut Query<&mut Inventory>,
        event_writer: &mut EventWriter<InventoryUpdateForProductionEvent>,
        item_manifest: &ItemManifest,
    ) -> TaskResult {
        match all_storages.get_many_mut([this_entity, self.target.into()]) {
            Ok([mut this_inv, mut other_inv]) => {
                match self.exchange_data {
                    ExchangeWareData::Buy(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Buy, amount, item_manifest);
                        other_inv.complete_order(item_id, TradeIntent::Sell, amount, item_manifest);
                    }
                    ExchangeWareData::Sell(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Sell, amount, item_manifest);
                        other_inv.complete_order(item_id, TradeIntent::Buy, amount, item_manifest);
                    }
                }
                event_writer.write(InventoryUpdateForProductionEvent::new(this_entity));
                event_writer.write(InventoryUpdateForProductionEvent::new(self.target.into()));
                TaskResult::Finished
            }
            Err(e) => {
                error!(
                    "Failed to execute ware exchange between {this_entity} and {:?}: {:?}",
                    self.target, e
                );
                TaskResult::Aborted
            }
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<ExchangeWares>>,
        simulation_time: Res<SimulationTime>,
        ships: Query<(Entity, &Self)>,
    ) {
        let now = simulation_time.now();
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<ExchangeWares>>::new()));

        ships
            .par_iter()
            .for_each(|(entity, task)| match task.run(now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<ExchangeWares>::new(entity.into())),
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut event_reader: EventReader<TaskCompletedEvent<ExchangeWares>>,
        mut all_ships_with_task: Query<&Self>,
        mut all_storages: Query<&mut Inventory>,
        mut event_writer: EventWriter<InventoryUpdateForProductionEvent>,
        item_manifest: Res<ItemManifest>,
    ) {
        for event in event_reader.read() {
            if let Ok(task) = all_ships_with_task.get_mut(event.entity.into()) {
                task.complete(
                    event.entity.into(),
                    &mut all_storages,
                    &mut event_writer,
                    &item_manifest,
                );
            } else {
                error!(
                    "Unable to find entity for ExchangeWares task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub fn on_task_started(
        mut query: Query<&mut Self>,
        mut finished_events: EventReader<TaskStartedEvent<ExchangeWares>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();
        for x in finished_events.read() {
            let Ok(mut created_component) = query.get_mut(x.entity.into()) else {
                continue;
            };

            created_component.finishes_at = now.add_seconds(2);
        }
    }

    pub(crate) fn cancel_task_inside_queue(
        mut events: EventReader<TaskCanceledWhileInQueueEvent<ExchangeWares>>,
        mut inventories: Query<&mut Inventory>,
    ) {
        for event in events.read() {}
    }

    pub(crate) fn abort_running_task() {
        panic!("Task cannot be properly aborted.");
    }
}

impl TaskCancellationForActiveTaskHandler<ExchangeWares, NoArgs> for ExchangeWares {}

#[derive(SystemParam)]
pub(crate) struct CancelExchangeWareArgs<'w, 's> {
    inventories: Query<'w, 's, &'static mut Inventory>,
}

impl<'w, 's> TaskCancellationForTaskInQueueHandler<ExchangeWares, CancelExchangeWareArgs<'w, 's>>
    for ExchangeWares
{
    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn on_task_cancellation_while_in_queue(
        event: &TaskCanceledWhileInQueueEvent<ExchangeWares>,
        args: &mut StaticSystemParam<CancelExchangeWareArgs>,
    ) -> Result<(), BevyError> {
        let exchange_data = &event.task_data.exchange_data;
        if let Ok(inventory) = args.inventories.get_mut(event.task_data.target.into()) {
            match exchange_data {
                ExchangeWareData::Buy(item_id, amount) => {}
                ExchangeWareData::Sell(item_id, amount) => {}
            }
        }

        if let Ok(inventory) = args.inventories.get_mut(event.entity.into()) {
            match exchange_data {
                ExchangeWareData::Buy(item_id, amount) => {}
                ExchangeWareData::Sell(item_id, amount) => {}
            }
        }
        todo!("Inventory plans need to get adjusted")
    }
}

#[derive(SystemParam)]
pub(crate) struct CreateExchangeWareArgs<'w, 's> {
    buy_orders: Query<'w, 's, &'static mut BuyOrders>,
    sell_orders: Query<'w, 's, &'static mut SellOrders>,
    inventories: Query<'w, 's, &'static mut Inventory>,
    item_manifest: Res<'w, ItemManifest>,
}

impl TaskCreationHandler<ExchangeWares, CreateExchangeWareArgs<'_, '_>> for ExchangeWares {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<ExchangeWares>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &mut StaticSystemParam<CreateExchangeWareArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let args = args.deref_mut();

        let mut new_tasks = create_preconditions_and_dock_at_entity(
            event.entity,
            event.task_data.target,
            task_queue,
            general_pathfinding_args,
        )?;

        // TODO: I think Inventory/Order updates should be handled within a separate event handler
        //      (which could also just listen to the task creation event)
        //      ...on the other hand side, task creation validation happening earlier and here is nice to have too?
        let Ok([mut this_inv, mut other_inv]) = args
            .inventories
            .get_many_mut([event.entity, event.task_data.target.into()])
        else {
            let this = args.inventories.get(event.entity);
            let other = args.inventories.get(event.task_data.target.into());

            let reason = if this.is_err() {
                if other.is_err() {
                    TaskCreationErrorReason::BothNotFound
                } else {
                    TaskCreationErrorReason::OwnEntityNotFound
                }
            } else if other.is_err() {
                TaskCreationErrorReason::TargetNotFound
            } else {
                TaskCreationErrorReason::UnspecifiedError
            };

            return Err(TaskCreationError {
                entity: event.entity,
                reason,
            }
            .into());
        };

        match event.task_data.exchange_data {
            ExchangeWareData::Buy(item_id, amount) => {
                this_inv.create_order(item_id, TradeIntent::Buy, amount, &args.item_manifest);
                other_inv.create_order(item_id, TradeIntent::Sell, amount, &args.item_manifest);
            }
            ExchangeWareData::Sell(item_id, amount) => {
                this_inv.create_order(item_id, TradeIntent::Sell, amount, &args.item_manifest);
                other_inv.create_order(item_id, TradeIntent::Buy, amount, &args.item_manifest);
            }
        }

        update_buy_and_sell_orders_for_entity(
            event.entity,
            &this_inv,
            &mut args.buy_orders,
            &mut args.sell_orders,
            &args.item_manifest,
        );
        update_buy_and_sell_orders_for_entity(
            event.task_data.target.into(),
            &other_inv,
            &mut args.buy_orders,
            &mut args.sell_orders,
            &args.item_manifest,
        );

        new_tasks.push_back(TaskKind::ExchangeWares {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}

fn update_buy_and_sell_orders_for_entity(
    entity: Entity,
    inventory: &Inventory,
    buy_orders: &mut Query<&mut BuyOrders>,
    sell_orders: &mut Query<&mut SellOrders>,
    item_manifest: &ItemManifest,
) {
    if let Ok(mut buy_orders) = buy_orders.get_mut(entity) {
        buy_orders.update(inventory, item_manifest);
    }
    if let Ok(mut sell_orders) = sell_orders.get_mut(entity) {
        sell_orders.update(inventory, item_manifest);
    }
}
