use crate::TaskComponent;
use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationError, TaskCreationErrorReason, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::tasks::send_completion_events;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_dock_at_entity;
use crate::utility::task_result::TaskResult;
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
use common::types::ship_tasks::{DockAtEntity, ExchangeWares};
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
pub(crate) struct TaskStartedArgs<'w> {
    simulation_time: Res<'w, SimulationTime>,
}

#[derive(SystemParam)]
pub(crate) struct TaskStartedArgsMut<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<ExchangeWares>>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = TaskStartedArgs<'w>;
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<ExchangeWares>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let mut created_component = args_mut.all_ships_with_task.get_mut(event.entity.into())?;
        created_component.finishes_at = args.simulation_time.now().add_seconds(2);
        Ok(())
    }
}

#[derive(SystemParam)]
pub(crate) struct TaskCompletedArgs<'w> {
    item_manifest: Res<'w, ItemManifest>,
}

#[derive(SystemParam)]
pub(crate) struct TaskCompletedArgsMut<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<ExchangeWares>>,
    all_storages: Query<'w, 's, &'static mut Inventory>,
    inventory_update_event_writer: EventWriter<'w, InventoryUpdateForProductionEvent>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = TaskCompletedArgs<'w>;
    type ArgsMut = TaskCompletedArgsMut<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<ExchangeWares>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let task = args_mut.all_ships_with_task.get_mut(event.entity.into())?;

        let [mut this_inv, mut other_inv] = args_mut
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
        args_mut
            .inventory_update_event_writer
            .write(InventoryUpdateForProductionEvent::new(event.entity.into()));
        args_mut
            .inventory_update_event_writer
            .write(InventoryUpdateForProductionEvent::new(task.target.into()));

        Ok(())
    }
}

#[derive(SystemParam)]
pub(crate) struct RunTasksArgs<'w> {
    simulation_time: Res<'w, SimulationTime>,
}

#[derive(SystemParam)]
pub(crate) struct RunTasksArgsMut<'w, 's> {
    ships: Query<'w, 's, (Entity, &'static ShipTask<ExchangeWares>)>,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = RunTasksArgs<'w>;
    type ArgsMut = RunTasksArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<ExchangeWares>>>>, BevyError> {
        let args_mut = args_mut.deref_mut();
        let now = args.simulation_time.now();
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<ExchangeWares>>::new()));

        args_mut
            .ships
            .par_iter()
            .for_each(|(entity, task)| match run_task(task, now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<ExchangeWares>::new(entity.into())),
            });

        Ok(task_completions)
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
    type Args = ();
    type ArgsMut = ();
}

#[derive(SystemParam)]
pub(crate) struct CancelExchangeWareArgsMut<'w, 's> {
    inventories: Query<'w, 's, &'static mut Inventory>,
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = ();
    type ArgsMut = CancelExchangeWareArgsMut<'w, 's>;

    fn can_task_be_cancelled_while_in_queue() -> bool {
        true
    }

    fn on_task_cancellation_while_in_queue(
        event: &TaskCanceledWhileInQueueEvent<ExchangeWares>,
        _: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let exchange_data = &event.task_data.exchange_data;
        if let Ok(inventory) = args_mut.inventories.get_mut(event.task_data.target.into()) {
            match exchange_data {
                ExchangeWareData::Buy(item_id, amount) => {}
                ExchangeWareData::Sell(item_id, amount) => {}
            }
        }

        if let Ok(inventory) = args_mut.inventories.get_mut(event.entity.into()) {
            match exchange_data {
                ExchangeWareData::Buy(item_id, amount) => {}
                ExchangeWareData::Sell(item_id, amount) => {}
            }
        }
        todo!("Inventory plans need to get adjusted")
    }
}

#[derive(SystemParam)]
pub(crate) struct CreateExchangeWareArgs<'w> {
    item_manifest: Res<'w, ItemManifest>,
}

#[derive(SystemParam)]
pub(crate) struct CreateExchangeWareArgsMut<'w, 's> {
    buy_orders: Query<'w, 's, &'static mut BuyOrders>,
    sell_orders: Query<'w, 's, &'static mut SellOrders>,
    inventories: Query<'w, 's, &'static mut Inventory>,
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, ExchangeWares> for ExchangeWares {
    type Args = CreateExchangeWareArgs<'w>;
    type ArgsMut = CreateExchangeWareArgsMut<'w, 's>;

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<ExchangeWares>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let args_mut = args_mut.deref_mut();

        let mut new_tasks = create_preconditions_and_dock_at_entity(
            event.entity,
            event.task_data.target,
            task_queue,
            general_pathfinding_args,
        )?;

        // TODO: I think Inventory/Order updates should be handled within a separate event handler
        //      (which could also just listen to the task creation event)
        //      ...on the other hand side, task creation validation happening earlier and here is nice to have too?
        let Ok([mut this_inv, mut other_inv]) = args_mut
            .inventories
            .get_many_mut([event.entity, event.task_data.target.into()])
        else {
            let this = args_mut.inventories.get(event.entity);
            let other = args_mut.inventories.get(event.task_data.target.into());

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
            &mut args_mut.buy_orders,
            &mut args_mut.sell_orders,
            &args.item_manifest,
        );
        update_buy_and_sell_orders_for_entity(
            event.task_data.target.into(),
            &other_inv,
            &mut args_mut.buy_orders,
            &mut args_mut.sell_orders,
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
