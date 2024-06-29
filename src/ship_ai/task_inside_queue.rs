use crate::ship_ai::tasks::ExchangeWares;
use crate::ship_ai::MoveToEntity;
use crate::utils::ExchangeWareData;
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
}

impl TaskInsideQueue {
    pub fn create_and_insert_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            TaskInsideQueue::ExchangeWares { target, data } => {
                entity_commands.insert(ExchangeWares {
                    finishes_at: 0,
                    target: *target,
                    data: *data,
                });
            }
            TaskInsideQueue::MoveToEntity { target } => {
                entity_commands.insert(MoveToEntity { target: *target });
            }
        }
    }
}
