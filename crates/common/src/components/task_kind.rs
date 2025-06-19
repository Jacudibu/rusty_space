use crate::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, Undock, UseGate,
};

/// Enum to differentiate between the different ship tasks.
///
/// If you implement logic for this, consider using a macro with [impl_all_task_kinds] in case every case does the same thing.
#[derive(Clone)]
pub enum TaskKind {
    /// Indicates that our ship is waiting for an external entity (e.g. a station or the player) to signal the ship to continue with its next task.
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
    MoveToPosition {
        data: MoveToPosition,
    },
    MoveToSector {
        data: MoveToSector,
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

/// Pass another macro in here with ($(($variant:ident, $snake_case_variant:ident)),*) => {...}
/// and it will populate the arguments with values for every [TaskKind]
///
/// see <https://lukaswirth.dev/tlborm/decl-macros/patterns/callbacks.html> to clarify the magic behind this
#[macro_export]
macro_rules! impl_all_task_kinds {
    ($callback:ident) => {
        $callback! {
            (AwaitingSignal, awaiting_signal),
            (Construct, construct),
            (DockAtEntity, dock_at_entity),
            (ExchangeWares, exchange_wares),
            (HarvestGas, harvest_gas),
            (MineAsteroid, mine_asteroid),
            (MoveToEntity, move_to_entity),
            (MoveToPosition, move_to_position),
            (MoveToSector, move_to_sector),
            (RequestAccess, request_access),
            (Undock, undock),
            (UseGate, use_gate)
        }
    };
}

pub use impl_all_task_kinds;
