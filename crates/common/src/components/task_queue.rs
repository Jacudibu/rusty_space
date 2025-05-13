use crate::components::task_kind::TaskKind;
use bevy::prelude::Component;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

/// Keeps track of the tasks a ship was... tasked to execute.
#[derive(Component, Default)]
pub struct TaskQueue {
    /// The currently active task. None means the ship is idle.
    /// These are also available as [ShipTask]<[TaskKindData]> Component on entities for better querying for ships with specific tasks.
    /// If you want to filter for idle ships, use [ShipIsIdleFilter].
    pub active_task: Option<TaskKind>,

    /// A queue of tasks which will be executed in order - usually first in, first out, though some situations might shift priorities.
    /// (once we implement that, it's probably best to add a function which will put the active task back into the queue)
    pub queue: VecDeque<TaskKind>,
}

impl Deref for TaskQueue {
    type Target = VecDeque<TaskKind>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for TaskQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
