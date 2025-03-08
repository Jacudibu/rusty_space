use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Component, Entity, EventWriter, Mut, Query};
use std::sync::{Arc, Mutex};

mod awaiting_signal;
mod build;
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
pub use {
    awaiting_signal::AwaitingSignal, build::Build, dock_at_entity::DockAtEntity,
    exchange_wares::ExchangeWares, harvest_gas::HarvestGas, mine_asteroid::MineAsteroid,
    move_to_entity::MoveToEntity, request_access::RequestAccess, undock::Undock, use_gate::UseGate,
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

/// Future Performance improvement: Once EventWriters can be written to in parallel, this could be run with a par_iter EventReader after all complete_task systems are done
/// https://github.com/bevyengine/bevy/issues/2648
/// Alternatively, `bevy_concurrent_event` would already enable that if we remove tasks the next frame (PreUpdate)
pub fn remove_task_and_add_next_in_queue<T: Component>(
    commands: &mut Commands,
    entity: Entity,
    queue: &mut Mut<TaskQueue>,
    now: CurrentSimulationTimestamp,
) {
    let mut entity_commands = commands.entity(entity);
    remove_task_and_add_next_in_queue_to_entity_commands::<T>(&mut entity_commands, queue, now);
}

pub fn remove_task_and_add_next_in_queue_to_entity_commands<T: Component>(
    entity_commands: &mut EntityCommands,
    queue: &mut Mut<TaskQueue>,
    now: CurrentSimulationTimestamp,
) {
    entity_commands.remove::<T>();
    queue.queue.pop_front();
    if let Some(next_task) = queue.front() {
        next_task.create_and_insert_component(entity_commands, now);
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
