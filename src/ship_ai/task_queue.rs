use crate::ship_ai::task_inside_queue::TaskInsideQueue;
use crate::utils::CurrentSimulationTimestamp;
use bevy::prelude::{Commands, Component, Entity};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

/// A queue of [ShipTask]s.
#[derive(Component)]
pub struct TaskQueue {
    pub queue: VecDeque<TaskInsideQueue>,
}

impl TaskQueue {
    pub fn with_capacity(capacity: usize) -> Self {
        TaskQueue {
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn apply(self, commands: &mut Commands, now: CurrentSimulationTimestamp, entity: Entity) {
        // TODO: Just reuse existing task queue, creating a new one every time is silly
        //       Behaviors will never run in parallel anyway
        let mut commands = commands.entity(entity);
        self.queue[0].create_and_insert_component(&mut commands, now);
        commands.insert(self);
    }
}

impl Deref for TaskQueue {
    type Target = VecDeque<TaskInsideQueue>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for TaskQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
