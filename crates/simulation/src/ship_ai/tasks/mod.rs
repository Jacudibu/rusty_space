use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Entity, EventWriter, Mut, Query, warn};
use std::sync::{Arc, Mutex};

mod awaiting_signal;
mod construct;
mod dock_at_entity;
mod exchange_wares;
mod harvest_gas;
mod mine_asteroid;
mod move_to_entity;
mod request_access;
mod undock;
mod use_gate;

use crate::ship_ai::ship_task::ShipTask;
use common::components::interaction_queue::InteractionQueue;
use common::components::task_queue::{TaskInsideQueue, TaskQueue};
use common::constants;
use common::events::task_events::{
    AllTaskStartedEventWriters, TaskCompletedEvent, TaskStartedEvent,
};
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTimestamp};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, ShipTaskData, Undock, UseGate,
};

pub fn send_completion_events<T: ShipTaskData + 'static>(
    mut event_writer: EventWriter<TaskCompletedEvent<T>>,
    task_completions: Arc<Mutex<Vec<TaskCompletedEvent<T>>>>,
) {
    match Arc::try_unwrap(task_completions) {
        Ok(task_completions) => {
            let batch = task_completions.into_inner().unwrap();
            if !batch.is_empty() {
                event_writer.write_batch(batch);
            }
        }
        Err(_) => {
            todo!()
        }
    }
}

pub fn remove_task_and_add_next_in_queue<T: ShipTaskData + 'static>(
    commands: &mut Commands,
    entity: Entity,
    queue: &mut Mut<TaskQueue>,
    now: CurrentSimulationTimestamp,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    let mut entity_commands = commands.entity(entity);
    remove_task_and_add_next_in_queue_to_entity_commands::<T>(
        entity,
        &mut entity_commands,
        queue,
        now,
        task_started_event_writers,
    );
}

pub fn remove_task_and_add_next_in_queue_to_entity_commands<T: ShipTaskData + 'static>(
    entity: Entity,
    entity_commands: &mut EntityCommands,
    queue: &mut Mut<TaskQueue>,
    now: CurrentSimulationTimestamp,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    entity_commands.remove::<ShipTask<T>>();
    queue.queue.pop_front();
    if let Some(next_task) = queue.front() {
        create_and_insert_component_from_task_inside_queue(
            next_task,
            entity.into(),
            entity_commands,
            now,
            task_started_event_writers,
        );
    }
}

/// Creates the Task Component for the first item in the queue to the provided entity.
/// Should be called by behaviors when transitioning away from idle states.
pub fn apply_new_task_queue(
    task_queue: &TaskQueue,
    commands: &mut Commands,
    now: CurrentSimulationTimestamp,
    entity: Entity,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    let mut commands = commands.entity(entity);
    create_and_insert_component_from_task_inside_queue(
        &task_queue.queue[0],
        entity.into(),
        &mut commands,
        now,
        task_started_event_writers,
    );
}

pub fn create_and_insert_component_from_task_inside_queue(
    task_inside_queue: &TaskInsideQueue,
    entity: ShipEntity,
    entity_commands: &mut EntityCommands,
    now: CurrentSimulationTimestamp,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    match task_inside_queue {
        TaskInsideQueue::ExchangeWares {
            target,
            exchange_data,
        } => {
            task_started_event_writers
                .exchange_wares
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<ExchangeWares>::new(ExchangeWares {
                finishes_at: SimulationTimestamp::MAX,
                target: *target,
                exchange_data: *exchange_data,
            }));
        }
        TaskInsideQueue::MoveToEntity {
            target,
            stop_at_target,
            distance_to_target: distance,
        } => {
            entity_commands.insert(ShipTask::<MoveToEntity>::new(MoveToEntity {
                target: *target,
                stop_at_target: *stop_at_target,
                desired_distance_to_target: *distance,
            }));
        }
        TaskInsideQueue::UseGate {
            enter_gate,
            exit_sector,
        } => {
            task_started_event_writers
                .use_gate
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<UseGate>::new(UseGate {
                progress: 0.0,
                traversal_state: Default::default(),
                exit_sector: *exit_sector,
                enter_gate: *enter_gate,
            }));
        }
        TaskInsideQueue::MineAsteroid { target, reserved } => {
            entity_commands.insert(ShipTask::<MineAsteroid>::new(MineAsteroid {
                target: *target,
                next_update: now.add_milliseconds(constants::ONE_SECOND_IN_MILLISECONDS),
                reserved_ore_amount: *reserved,
            }));
        }
        TaskInsideQueue::HarvestGas { target, gas } => {
            entity_commands.insert(ShipTask::<HarvestGas>::new(HarvestGas {
                target: *target,
                gas: *gas,
                next_update: now.add_milliseconds(constants::ONE_SECOND_IN_MILLISECONDS),
            }));
        }
        TaskInsideQueue::AwaitingSignal { target: from } => {
            entity_commands.insert(ShipTask::<AwaitingSignal>::new(AwaitingSignal {
                from: *from,
            }));
        }
        TaskInsideQueue::Construct { target } => {
            task_started_event_writers
                .construct
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<Construct>::new(Construct { target: *target }));
        }
        TaskInsideQueue::RequestAccess { target } => {
            entity_commands.insert(ShipTask::<RequestAccess>::new(RequestAccess {
                target: *target,
            }));
        }
        TaskInsideQueue::DockAtEntity { target } => {
            entity_commands.insert(ShipTask::<DockAtEntity>::new(DockAtEntity {
                target: *target,
            }));
        }
        TaskInsideQueue::Undock => {
            task_started_event_writers
                .undock
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<Undock>::new(Undock {
                start_position: None,
            }));
        }
    }
}

#[inline]
pub fn finish_interaction(
    queue_entity: Entity,
    interaction_queues: &mut Query<&mut InteractionQueue>,
    signal_writer: &mut EventWriter<TaskCompletedEvent<AwaitingSignal>>,
) {
    let Ok(mut queue_entity) = interaction_queues.get_mut(queue_entity) else {
        warn!("Was unable to find queue entity in finish interaction!");
        return;
    };

    queue_entity.finish_interaction(signal_writer);
}
