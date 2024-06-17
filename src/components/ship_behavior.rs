use bevy::prelude::Component;

/// Indicates how a Ship should be controlled by the AI.
#[derive(Component)]
pub enum ShipBehavior {
    /// The Ship won't do anything unless specifically ordered to do so.
    HoldPosition,

    /// The Ship will buy and sell wares as specified in AutoTradeData.
    AutoTrade(AutoTradeData),
}

pub struct AutoTradeData {}
