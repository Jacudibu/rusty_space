use crate::game_data::ItemId;
use crate::simulation::prelude::SimulationTimestamp;
use crate::simulation::ship_ai::behaviors::auto_construct::AutoConstructBehavior;
use crate::simulation::ship_ai::behaviors::auto_harvest::AutoHarvestBehavior;
use crate::simulation::ship_ai::{AutoMineBehavior, AutoMineState, AutoTradeBehavior};
use bevy::ecs::system::EntityCommands;

pub(crate) mod auto_construct;
pub mod auto_harvest;
pub mod auto_mine;
pub mod auto_trade;

pub enum BehaviorBuilder {
    AutoTrade {
        next_idle_update: SimulationTimestamp,
    },
    AutoConstruct {
        next_idle_update: SimulationTimestamp,
    },
    AutoMine {
        next_idle_update: SimulationTimestamp,
        mined_ore: ItemId,
        state: AutoMineState,
    },
    AutoHarvest {
        next_idle_update: SimulationTimestamp,
        state: AutoMineState,
        harvested_gas: ItemId,
    },
}

impl BehaviorBuilder {
    // if we ever need to use something similar at multiple locations, there is an example
    // on how to generically implement this as an EntityCommands trait over at
    // the end of https://github.com/bevyengine/bevy/discussions/11409
    pub fn build_and_add_default_component(self, mut entity_commands: EntityCommands) {
        match self {
            BehaviorBuilder::AutoTrade { next_idle_update } => {
                entity_commands.insert(AutoTradeBehavior { next_idle_update })
            }
            BehaviorBuilder::AutoConstruct { next_idle_update } => {
                entity_commands.insert(AutoConstructBehavior { next_idle_update })
            }
            BehaviorBuilder::AutoMine {
                next_idle_update,
                mined_ore,
                state,
            } => entity_commands.insert(AutoMineBehavior {
                next_idle_update,
                mined_ore,
                state,
            }),
            BehaviorBuilder::AutoHarvest {
                next_idle_update,
                state,
                harvested_gas,
            } => entity_commands.insert(AutoHarvestBehavior {
                next_idle_update,
                state,
                harvested_gas,
            }),
        };
    }
}
