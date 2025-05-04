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

use common::simulation_time::CurrentSimulationTimestamp;

use crate::simulation::interaction_queue::InteractionQueue;
use crate::simulation::prelude::{TaskCompletedEvent, TaskQueue};
use crate::simulation::ship_ai::TaskComponent;
use crate::simulation::ship_ai::task_events::AllTaskStartedEventWriters;
pub use {
    awaiting_signal::AwaitingSignal, construct::Construct, dock_at_entity::DockAtEntity,
    exchange_wares::ExchangeWares, harvest_gas::HarvestGas, mine_asteroid::MineAsteroid,
    move_to_entity::MoveToEntity, request_access::RequestAccess, undock::Undock, use_gate::UseGate,
};

pub fn send_completion_events<T: TaskComponent>(
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

pub fn remove_task_and_add_next_in_queue<T: TaskComponent>(
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

pub fn remove_task_and_add_next_in_queue_to_entity_commands<T: TaskComponent>(
    entity: Entity,
    entity_commands: &mut EntityCommands,
    queue: &mut Mut<TaskQueue>,
    now: CurrentSimulationTimestamp,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    entity_commands.remove::<T>();
    queue.queue.pop_front();
    if let Some(next_task) = queue.front() {
        next_task.create_and_insert_component(
            entity.into(),
            entity_commands,
            now,
            task_started_event_writers,
        );
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
