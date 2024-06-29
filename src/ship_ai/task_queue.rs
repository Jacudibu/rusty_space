use crate::ship_ai::task_inside_queue::TaskInsideQueue;
use bevy::prelude::Component;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

/// A queue of [ShipTask]s.
#[derive(Component)]
pub struct TaskQueue {
    pub queue: VecDeque<TaskInsideQueue>,
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
