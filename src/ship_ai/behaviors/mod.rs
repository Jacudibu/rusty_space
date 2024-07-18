use crate::ship_ai::{AutoMineBehavior, AutoMineState, AutoTradeBehavior};
use crate::utils::SimulationTimestamp;
use bevy::ecs::system::EntityCommands;

pub mod auto_mine;
pub mod auto_trade;

pub enum BehaviorBuilder {
    AutoTrade {
        next_idle_update: SimulationTimestamp,
    },
    AutoMine {
        next_idle_update: SimulationTimestamp,
        state: AutoMineState,
    },
}

impl BehaviorBuilder {
    // if we ever need to use something similar at multiple locations, there is an example
    // on how to generically implement this as an EntityCommands trait over at
    // the end of https://github.com/bevyengine/bevy/discussions/11409
    pub fn build_and_add_default_component(&self, mut entity_commands: EntityCommands) {
        match self {
            BehaviorBuilder::AutoTrade { next_idle_update } => {
                entity_commands.insert(AutoTradeBehavior {
                    next_idle_update: *next_idle_update,
                })
            }
            BehaviorBuilder::AutoMine {
                next_idle_update,
                state,
            } => entity_commands.insert(AutoMineBehavior {
                next_idle_update: *next_idle_update,
                state: *state,
            }),
        };
    }
}
