use crate::game_data::ItemId;
use crate::types::entity_wrappers::{
    AsteroidEntity, CelestialEntity, ConstructionSiteEntity, GateEntity, SectorEntity, TypedEntity,
};
use crate::types::exchange_ware_data::ExchangeWareData;
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
        target: TypedEntity,
    },
    Construct {
        target: ConstructionSiteEntity,
    },
    /// The ship will tell the provided entity that it wants to access it.
    /// Depending on how busy the target is, it will either tell us to go straight ahead and proceed with the next task or enter the ship into a queue, causing this task to be replaced by [TaskInsideQueue::AwaitingSignal].
    RequestAccess {
        target: TypedEntity,
    },
    DockAtEntity {
        target: TypedEntity,
    },
    Undock,
    ExchangeWares {
        target: TypedEntity,
        exchange_data: ExchangeWareData,
    },
    MoveToEntity {
        target: TypedEntity,
        /// If true, the ship will not slow down when approaching the target, meaning it will effectively fly right through it as the task completes.
        stop_at_target: bool,
        /// The desired distance to the target, in case the ship is not supposed to stop right on top of it but a bit earlier.
        distance_to_target: f32,
    },
    UseGate {
        enter_gate: GateEntity,
        exit_sector: SectorEntity,
    },
    MineAsteroid {
        target: AsteroidEntity,
        /// The amount of ore inside the asteroid which is reserved for our ship.
        reserved: u32,
    },
    HarvestGas {
        target: CelestialEntity,
        /// The [ItemId] for the gas which is supposed to be harvested. We need this since Gas Giants may contain multiple gases.
        gas: ItemId,
    },
}
