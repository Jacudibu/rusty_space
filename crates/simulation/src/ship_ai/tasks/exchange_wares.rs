use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_creation::{
    TaskCreation, TaskCreationError, TaskCreationErrorReason, create_tasks_to_move_to_sector,
};
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventReader, EventWriter, Query, Res, Transform, error};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{
    BuyOrders, InSector, Inventory, IsDocked, Sector, SellOrders, TradeOrder,
};
use common::constants;
use common::events::inventory_update_for_production_event::InventoryUpdateForProductionEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileInQueueEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTime};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::{SectorEntity, TypedEntity};
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_tasks;
use common::types::ship_tasks::{ExchangeWares, Undock};
use common::types::trade_intent::TradeIntent;
use std::collections::VecDeque;
use std::ops::{DerefMut, Not};
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
        for event in events.read() {
            let exchange_data = &event.task_data.exchange_data;
            if let Ok(inventory) = inventories.get_mut(event.task_data.target.into()) {
                match exchange_data {
                    ExchangeWareData::Buy(item_id, amount) => {}
                    ExchangeWareData::Sell(item_id, amount) => {}
                }
            }

            if let Ok(inventory) = inventories.get_mut(event.entity.into()) {
                match exchange_data {
                    ExchangeWareData::Buy(item_id, amount) => {}
                    ExchangeWareData::Sell(item_id, amount) => {}
                }
            }
            todo!("Inventory plans need to get adjusted")
        }
    }

    pub(crate) fn abort_running_task() {
        panic!("Task cannot be properly aborted.");
    }
}

#[derive(SystemParam)]
pub(crate) struct CreateExchangeWareArgs<'w, 's> {
    relevant_entities: Query<'w, 's, (&'static InSector, &'static Transform)>,
    is_docked: Query<'w, 's, &'static IsDocked>,
    all_sectors: Query<'w, 's, &'static Sector>,
    all_transforms: Query<'w, 's, &'static SimulationTransform>,
    buy_orders: Query<'w, 's, &'static mut BuyOrders>,
    sell_orders: Query<'w, 's, &'static mut SellOrders>,
    inventories: Query<'w, 's, &'static mut Inventory>,
    item_manifest: Res<'w, ItemManifest>,
}

struct SectorAndDockingStatus {
    docked_at: Option<TypedEntity>,
    sector: SectorEntity,
}

impl TaskCreation<ExchangeWares, CreateExchangeWareArgs<'_, '_>> for ExchangeWares {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<ExchangeWares>,
        task_queue: &TaskQueue,
        args: &mut StaticSystemParam<CreateExchangeWareArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let args = args.deref_mut();

        // TODO: Extract this. Also needs to be used by MoveToSector
        let sector_and_docking_status: SectorAndDockingStatus =
            if let Some(last_task) = task_queue.queue.back() {
                get_task_end_sector_and_position(&args.relevant_entities, last_task)?
            } else if let Some(active_task) = &task_queue.active_task {
                get_task_end_sector_and_position(&args.relevant_entities, active_task)?
            } else {
                let Ok((this_sector, _)) = args.relevant_entities.get(event.entity) else {
                    return Err(TaskCreationError {
                        entity: event.entity,
                        reason: TaskCreationErrorReason::OwnEntityNotFound,
                    }
                    .into());
                };

                SectorAndDockingStatus {
                    sector: this_sector.sector,
                    docked_at: args.is_docked.get(event.entity).map(|x| x.at).ok(),
                }
            };

        let mut new_tasks = VecDeque::default();

        // TODO: Either always add undock and skip it if we aren't docked,
        //       OR check if we are docked as a precondition in MoveTo[X] Commands.
        //      Probably better than checking it here.
        if let Some(docked_at) = sector_and_docking_status.docked_at {
            new_tasks.push_back(TaskKind::Undock {
                data: Undock {
                    start_position: None,
                    from: docked_at,
                },
            })
        }

        // TODO: We also need to kick out docked idle ships in case something else *needs* to dock for another task

        let Ok((target_sector, target_transform)) =
            args.relevant_entities.get(event.task_data.target.into())
        else {
            return Err(TaskCreationError {
                entity: event.entity,
                reason: TaskCreationErrorReason::TargetNotFound,
            }
            .into());
        };

        create_tasks_to_move_to_sector(
            event.entity,
            sector_and_docking_status.sector,
            target_sector.sector,
            Some(target_transform.translation.truncate()),
            &args.all_sectors,
            &args.all_transforms,
            &mut new_tasks,
        )?;

        new_tasks.push_back(TaskKind::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: event.task_data.target,
                stop_at_target: true,
                desired_distance_to_target: constants::DOCKING_DISTANCE_TO_STATION,
            },
        });
        new_tasks.push_back(TaskKind::RequestAccess {
            data: ship_tasks::RequestAccess {
                target: event.task_data.target,
            },
        });
        new_tasks.push_back(TaskKind::DockAtEntity {
            data: ship_tasks::DockAtEntity {
                target: event.task_data.target,
            },
        });

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

fn get_sector(
    entity: Entity,
    in_sector_query: &Query<(&InSector, &Transform)>,
) -> Result<SectorEntity, BevyError> {
    Ok(in_sector_query.get(entity)?.0.sector)
}

fn get_task_end_sector_and_position(
    in_sector_query: &Query<(&InSector, &Transform)>,
    relevant_task: &TaskKind,
) -> Result<SectorAndDockingStatus, BevyError> {
    let result = match relevant_task {
        TaskKind::AwaitingSignal { .. } => {
            todo!(
                "This should never be the last task in a queue... actually, it might once players can send signals"
            )
        }
        TaskKind::Construct { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::RequestAccess { .. } => {
            todo!("This should never be the last task in a queue")
        }
        TaskKind::DockAtEntity { data } => SectorAndDockingStatus {
            docked_at: Some(data.target),
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::Undock { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.from.into(), in_sector_query)?,
        },
        TaskKind::ExchangeWares { data } => SectorAndDockingStatus {
            docked_at: Some(data.target), // TODO: Both are highly dynamic once target can be a ship
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::MoveToEntity { data } => SectorAndDockingStatus {
            docked_at: None, // TODO: Both are highly dynamic if target is a ship
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::MoveToPosition { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.sector_position.sector.into(), in_sector_query)?,
        },
        TaskKind::UseGate { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.exit_sector.into(), in_sector_query)?,
        },
        TaskKind::MineAsteroid { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
        TaskKind::HarvestGas { data } => SectorAndDockingStatus {
            docked_at: None,
            sector: get_sector(data.target.into(), in_sector_query)?,
        },
    };

    Ok(result)
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
