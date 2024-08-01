use crate::simulation::prelude::{CurrentSimulationTimestamp, SimulationTimestamp};
use crate::simulation::ship_ai::tasks;
use crate::utils::{AsteroidEntity, ExchangeWareData, PlanetEntity, TypedEntity};
use crate::utils::{GateEntity, SectorEntity};
use bevy::ecs::system::EntityCommands;

/// Defines a Task inside the [TaskQueue]. New task components can be created from these.
pub enum TaskInsideQueue {
    AwaitingSignal,
    RequestAccess {
        target: TypedEntity,
    },
    ExchangeWares {
        target: TypedEntity,
        data: ExchangeWareData,
    },
    MoveToEntity {
        target: TypedEntity,
        stop_at_target: bool,
    },
    UseGate {
        enter_gate: GateEntity,
        exit_sector: SectorEntity,
    },
    MineAsteroid {
        target: AsteroidEntity,
        reserved: u32,
    },
    HarvestGas {
        target: PlanetEntity,
    },
}

impl TaskInsideQueue {
    pub fn create_and_insert_component(
        &self,
        entity_commands: &mut EntityCommands,
        now: CurrentSimulationTimestamp,
    ) {
        match self {
            TaskInsideQueue::ExchangeWares { target, data } => {
                entity_commands.insert(tasks::ExchangeWares {
                    finishes_at: SimulationTimestamp::MAX,
                    target: *target,
                    data: *data,
                });
            }
            TaskInsideQueue::MoveToEntity {
                target,
                stop_at_target,
            } => {
                entity_commands.insert(tasks::MoveToEntity {
                    target: *target,
                    stop_at_target: *stop_at_target,
                });
            }
            TaskInsideQueue::UseGate {
                enter_gate,
                exit_sector,
            } => {
                entity_commands.insert(tasks::UseGate {
                    progress: 0.0,
                    traversal_state: Default::default(),
                    exit_sector: *exit_sector,
                    enter_gate: *enter_gate,
                });
            }
            TaskInsideQueue::MineAsteroid { target, reserved } => {
                entity_commands.insert(tasks::MineAsteroid::new(*target, now, *reserved));
            }
            TaskInsideQueue::HarvestGas { target } => {
                entity_commands.insert(tasks::HarvestGas::new(*target, now));
            }
            TaskInsideQueue::AwaitingSignal => {
                entity_commands.insert(tasks::AwaitingSignal {});
            }
            TaskInsideQueue::RequestAccess { target } => {
                entity_commands.insert(tasks::RequestAccess::new(*target));
            }
        }
    }
}
