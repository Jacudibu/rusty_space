use crate::sectors::{GateId, SectorId};
use crate::ship_ai::tasks::{ExchangeWares, UseGate};
use crate::ship_ai::MoveToEntity;
use crate::utils::{ExchangeWareData, SimulationTimestamp};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

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
        exit_sector: SectorId,
        exit_gate: GateId,
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
                exit_sector,
                exit_gate,
            } => {
                entity_commands.insert(UseGate {
                    started_at: SimulationTimestamp::MIN,
                    finishes_at: SimulationTimestamp::MAX,
                    exit_sector: *exit_sector,
                    exit_gate: *exit_gate,
                });
            }
        }
    }
}
