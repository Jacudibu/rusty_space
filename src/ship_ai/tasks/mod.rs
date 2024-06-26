use bevy::prelude::{Commands, Component, Entity, EventWriter, Mut};
use std::sync::{Arc, Mutex};

mod exchange_wares;
mod idle;
mod move_to_entity;
mod use_gate;

use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
pub use {
    exchange_wares::ExchangeWares, idle::Idle, move_to_entity::MoveToEntity, use_gate::UseGate,
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
pub fn remove_task_and_add_new_one<T: Component>(
    commands: &mut Commands,
    entity: Entity,
    queue: &mut Mut<TaskQueue>,
) {
    queue.queue.pop_front();
    let mut entity_commands = commands.entity(entity);
    entity_commands.remove::<T>();
    if let Some(next_task) = queue.front() {
        next_task.create_and_insert_component(&mut entity_commands);
    } else {
        entity_commands.insert(Idle::default());
    }
}
