use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

use crate::components::{GateEntity, SectorEntity};
use crate::ship_ai::tasks::{ExchangeWares, UseGate};
use crate::ship_ai::MoveToEntity;
use crate::utils::{ExchangeWareData, SimulationTimestamp};

/// Defines a Task inside the [TaskQueue]. New task components can be created from these.
pub enum TaskInsideQueue {
    ExchangeWares {
        target: Entity,
        data: ExchangeWareData,
    },
    MoveToEntity {
        target: Entity,
    },
    UseGate {
        enter_gate: GateEntity,
        exit_sector: SectorEntity,
    },
}

impl TaskInsideQueue {
    pub fn create_and_insert_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            TaskInsideQueue::ExchangeWares { target, data } => {
                entity_commands.insert(ExchangeWares {
                    finishes_at: SimulationTimestamp::MAX,
                    target: *target,
                    data: *data,
                });
            }
            TaskInsideQueue::MoveToEntity { target } => {
                entity_commands.insert(MoveToEntity { target: *target });
            }
            TaskInsideQueue::UseGate {
                enter_gate,
                exit_sector,
            } => {
                entity_commands.insert(UseGate {
                    progress: 0.0,
                    exit_sector: *exit_sector,
                    enter_gate: *enter_gate,
                });
            }
        }
    }
}
