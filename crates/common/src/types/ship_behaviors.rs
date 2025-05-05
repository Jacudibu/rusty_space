use crate::game_data::ItemId;
use crate::types::auto_mine_state::AutoMineState;
use bevy::prelude::Component;

/// Marker trait to define that a struct may be used as a ShipBehavior during simulation.
pub trait ShipBehaviorData: Send + Sync {}

/// Ships with this behavior will automatically search out construction sites and share their build power.
pub struct AutoConstructBehavior {}
impl ShipBehaviorData for AutoConstructBehavior {}

/// Ships with this behavior will alternate between harvesting gas from gas giants and selling their inventory to stations.
pub struct AutoHarvestBehavior {
    // TODO: Maybe(?) could just be AutoMineBehavior<T> with T: MineAsteroid | HarvestGas
    pub harvested_gas: ItemId,
    pub state: AutoMineState,
}
impl ShipBehaviorData for AutoHarvestBehavior {}

/// Ships with this behavior will alternate between mining asteroids and selling off their inventory.
pub struct AutoMineBehavior {
    pub mined_ore: ItemId,
    pub state: AutoMineState,
}
impl ShipBehaviorData for AutoMineBehavior {}

/// Ships with this behavior will attempt to buy low and sell high.
#[derive(Component)]
pub struct AutoTradeBehavior {}
impl ShipBehaviorData for AutoTradeBehavior {}
