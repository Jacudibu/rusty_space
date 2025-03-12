use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Component, Entity, EventWriter, Mut, Query};
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

use crate::simulation::prelude::{CurrentSimulationTimestamp, TaskFinishedEvent, TaskQueue};

use crate::components::InteractionQueue;
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
pub use {
    awaiting_signal::AwaitingSignal, construct::ConstructTaskComponent,
    dock_at_entity::DockAtEntity, exchange_wares::ExchangeWares, harvest_gas::HarvestGas,
    mine_asteroid::MineAsteroid, move_to_entity::MoveToEntity, request_access::RequestAccess,
    undock::Undock, use_gate::UseGate,
};

pub fn send_completion_events<T: Component>(
    mut event_writer: EventWriter<TaskFinishedEvent<T>>,
    task_completions: Arc<Mutex<Vec<TaskFinishedEvent<T>>>>,
) {
    match Arc::try_unwrap(task_completions) {
        Ok(task_completions) => {
            let batch = task_completions.into_inner().unwrap();
            if !batch.is_empty() {
                event_writer.send_batch(batch);
            }
        }
        Err(_) => {
            todo!()
        }
    }
}

pub fn remove_task_and_add_next_in_queue<T: Component>(
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

pub fn remove_task_and_add_next_in_queue_to_entity_commands<T: Component>(
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
    signal_writer: &mut EventWriter<TaskFinishedEvent<AwaitingSignal>>,
) {
    interaction_queues
        .get_mut(queue_entity)
        .unwrap()
        .finish_interaction(signal_writer);
}
