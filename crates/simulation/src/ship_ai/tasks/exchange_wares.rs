use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::ship_ai::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::ship_ai::task_completed::TaskCompletedEventHandler;
use crate::ship_ai::task_creation::{
    GeneralPathfindingArgs, TaskCreationError, TaskCreationErrorReason, TaskCreationEventHandler,
    create_preconditions_and_dock_at_entity,
};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::task_runner::TaskRunner;
use crate::ship_ai::task_started::TaskStartedEventHandler;
use crate::ship_ai::tasks::send_completion_events;
use crate::ship_ai::{NoArgs, TaskComponent};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventWriter, Query, Res};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{BuyOrders, Inventory, SellOrders, TradeOrder};
use common::constants::BevyResult;
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

#[derive(SystemParam)]
pub(crate) struct TaskStartedArgs<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<ExchangeWares>>,
    simulation_time: Res<'w, SimulationTime>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = TaskStartedArgs<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<ExchangeWares>,
        args: &mut StaticSystemParam<Self::Args>,
    ) -> Result<(), BevyError> {
        let finishes_at = args.simulation_time.now().add_seconds(2);
        let mut created_component = args.all_ships_with_task.get_mut(event.entity.into())?;
        created_component.finishes_at = finishes_at;
        Ok(())
    }
}

#[derive(SystemParam)]
pub(crate) struct TaskCompletedArgs<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<ExchangeWares>>,
    all_storages: Query<'w, 's, &'static mut Inventory>,
    inventory_update_event_writer: EventWriter<'w, InventoryUpdateForProductionEvent>,
    item_manifest: Res<'w, ItemManifest>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = TaskCompletedArgs<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<ExchangeWares>,
        args: &mut StaticSystemParam<Self::Args>,
    ) -> Result<(), BevyError> {
        let args = args.deref_mut();
        let task = args.all_ships_with_task.get_mut(event.entity.into())?;

        let [mut this_inv, mut other_inv] = args
            .all_storages
            .get_many_mut([event.entity.into(), task.target.into()])?;
        match task.exchange_data {
            ExchangeWareData::Buy(item_id, amount) => {
                this_inv.complete_order(item_id, TradeIntent::Buy, amount, &args.item_manifest);
                other_inv.complete_order(item_id, TradeIntent::Sell, amount, &args.item_manifest);
            }
            ExchangeWareData::Sell(item_id, amount) => {
                this_inv.complete_order(item_id, TradeIntent::Sell, amount, &args.item_manifest);
                other_inv.complete_order(item_id, TradeIntent::Buy, amount, &args.item_manifest);
            }
        }
        args.inventory_update_event_writer
            .write(InventoryUpdateForProductionEvent::new(event.entity.into()));
        args.inventory_update_event_writer
            .write(InventoryUpdateForProductionEvent::new(task.target.into()));

        Ok(())
    }
}

#[derive(SystemParam)]
pub(crate) struct RunTasksArgs<'w, 's> {
    simulation_time: Res<'w, SimulationTime>,
    ships: Query<'w, 's, (Entity, &'static ShipTask<ExchangeWares>)>,
}

impl<'w, 's> TaskRunner<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = RunTasksArgs<'w, 's>;

    fn run_all_tasks(
        event_writer: EventWriter<TaskCompletedEvent<ExchangeWares>>,
        mut args: StaticSystemParam<Self::Args>,
    ) -> BevyResult {
        let args = args.deref_mut();
        let now = args.simulation_time.now();
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<ExchangeWares>>::new()));

        args.ships
            .par_iter()
            .for_each(|(entity, task)| match run_task(task, now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<ExchangeWares>::new(entity.into())),
            });

        send_completion_events(event_writer, task_completions);

        Ok(())
    }
}

fn run_task(task: &ShipTask<ExchangeWares>, now: CurrentSimulationTimestamp) -> TaskResult {
    if now.has_not_passed(task.finishes_at) {
        TaskResult::Ongoing
    } else {
        TaskResult::Finished
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = NoArgs;
}

#[derive(SystemParam)]
pub(crate) struct CancelExchangeWareArgs<'w, 's> {
    inventories: Query<'w, 's, &'static mut Inventory>,
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = CancelExchangeWareArgs<'w, 's>;

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn on_task_cancellation_while_in_queue(
        event: &TaskCanceledWhileInQueueEvent<ExchangeWares>,
        args: &mut StaticSystemParam<Self::Args>,
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

impl<'w, 's> TaskCreationEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = CreateExchangeWareArgs<'w, 's>;

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<ExchangeWares>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &mut StaticSystemParam<Self::Args>,
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
