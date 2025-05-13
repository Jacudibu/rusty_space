use crate::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    RequestAccess, Undock, UseGate,
};

/// Enum to differentiate between the different ship tasks.
#[derive(Clone)]
pub enum TaskKind {
    /// Indicates that our ship is waiting for an external entity (e.g. a station or the player) to signal the ship to continue with it next task.
    AwaitingSignal {
        data: AwaitingSignal,
    },
    Construct {
        data: Construct,
    },
    /// The ship will tell the provided entity that it wants to access it.
    /// Depending on how busy the target is, it will either tell us to go straight ahead and proceed with the next task or enter the ship into a queue, causing this task to be replaced by [TaskKind::AwaitingSignal].
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
