use crate::ship_ai::{AutoMineBehavior, AutoTradeBehavior};
use bevy::ecs::system::EntityCommands;

pub mod auto_mine;
pub mod auto_trade;

// TODO: There's probably better ways to persist behavior data than managing an extra enum
pub enum BehaviorBuilder {
    AutoTrade,
    AutoMine,
}

impl BehaviorBuilder {
    pub fn build_and_add_default_component(&self, mut entity_commands: EntityCommands) {
        match self {
            BehaviorBuilder::AutoTrade => entity_commands.insert(AutoTradeBehavior::default()),
            BehaviorBuilder::AutoMine => entity_commands.insert(AutoMineBehavior::default()),
        };
    }
}
