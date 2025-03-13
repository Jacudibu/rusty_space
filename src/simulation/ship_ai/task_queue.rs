use crate::simulation::prelude::{CurrentSimulationTimestamp, TaskInsideQueue};
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
use bevy::prelude::{Commands, Component, Entity};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

/// A queue of [ShipTask]s.
#[derive(Component)]
pub struct TaskQueue {
    pub queue: VecDeque<TaskInsideQueue>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            queue: VecDeque::new(),
        }
    }

    /// Creates the Task Component for the first item in the queue to the provided entity.
    /// Should be called by behaviors when transitioning away from idle states.
    pub fn apply(
        &self,
        commands: &mut Commands,
        now: CurrentSimulationTimestamp,
        entity: Entity,
        task_started_event_writers: &mut AllTaskStartedEventWriters,
    ) {
        let mut commands = commands.entity(entity);
        self.queue[0].create_and_insert_component(
            entity.into(),
            &mut commands,
            now,
            task_started_event_writers,
        );
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
