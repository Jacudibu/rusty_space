use crate::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, Undock, UseGate,
};
use bevy::prelude::Component;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

/// A queue of [ShipTask]s.
#[derive(Component, Default)]
pub struct TaskQueue {
    /// A queue of tasks which will be executed in order.
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

/// Defines a Task inside the [TaskQueue]. New task components can be created from these.
pub enum TaskInsideQueue {
    /// Indicates that our ship is waiting for an external entity (e.g. a station or the player) to signal the ship to continue with it next task.
    AwaitingSignal {
        data: AwaitingSignal,
    },
    Construct {
        data: Construct,
    },
    /// The ship will tell the provided entity that it wants to access it.
    /// Depending on how busy the target is, it will either tell us to go straight ahead and proceed with the next task or enter the ship into a queue, causing this task to be replaced by [TaskInsideQueue::AwaitingSignal].
    RequestAccess {
        data: RequestAccess,
    },
    DockAtEntity {
        data: DockAtEntity,
    },
    Undock {
        data: Undock,
    },
    ExchangeWares {
        data: ExchangeWares,
    },
    MoveToEntity {
        data: MoveToEntity,
    },
    UseGate {
        data: UseGate,
    },
    MineAsteroid {
        data: MineAsteroid,
    },
    HarvestGas {
        data: HarvestGas,
    },
}
