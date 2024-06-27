use crate::game_data::ItemId;
use bevy::prelude::{Component, Entity};
use std::collections::VecDeque;

/// A single Task which can be scheduled for individual ships.
pub enum ShipTask {
    DoNothing,
    /// Move to target Entity's position. Stop when there.
    MoveTo(Entity),

    /// Transfer wares between the ship and the target entity's storage
    ExchangeWares(Entity, ExchangeWareData),
}

/// A queue of [ShipTasks].
#[derive(Component)]
pub struct TaskQueue {
    pub queue: VecDeque<ShipTask>,
}

pub enum ExchangeWareData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}
