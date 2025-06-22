use crate::components::ship_behavior::ShipBehavior;
use crate::game_data::ItemId;
use crate::types::auto_mine_state::AutoMineState;
use crate::types::ship_behaviors::{
    AutoConstructBehavior, AutoHarvestBehavior, AutoMineBehavior, AutoTradeBehavior,
    HoldPositionBehavior,
};
use bevy::prelude::EntityCommands;

/// TODO: Rename to BehaviorKind
/// Enum representing all ship behaviors.
pub enum BehaviorBuilder {
    AutoTrade,
    AutoConstruct,
    AutoMine {
        mined_ore: ItemId,
        state: AutoMineState,
    },
    AutoHarvest {
        state: AutoMineState,
        harvested_gas: ItemId,
    },
    HoldPosition,
}

impl BehaviorBuilder {
    // if we ever need to use something similar at multiple locations, there is an example
    // on how to generically implement this as an EntityCommands trait over at
    // the end of https://github.com/bevyengine/bevy/discussions/11409
    pub fn build_and_add_default_component(self, mut entity_commands: EntityCommands) {
        match self {
            BehaviorBuilder::AutoTrade => {
                entity_commands.insert(ShipBehavior::new(AutoTradeBehavior {}));
            }
            BehaviorBuilder::AutoConstruct => {
                entity_commands.insert(ShipBehavior::new(AutoConstructBehavior {}));
            }
            BehaviorBuilder::AutoMine { mined_ore, state } => {
                entity_commands.insert(ShipBehavior::new(AutoMineBehavior { mined_ore, state }));
            }
            BehaviorBuilder::AutoHarvest {
                state,
                harvested_gas,
            } => {
                entity_commands.insert(ShipBehavior::new(AutoHarvestBehavior {
                    state,
                    harvested_gas,
                }));
            }
            BehaviorBuilder::HoldPosition => {
                entity_commands.insert(ShipBehavior::new(HoldPositionBehavior {}));
            }
        };
    }
}
