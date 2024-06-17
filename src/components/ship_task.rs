use bevy::prelude::{Component, Entity};

/// A single Task which can be scheduled for individual ships.
pub enum ShipTask {
    DoNothing,
    MoveTo(Entity),
}

/// A queue of [ShipTasks].
#[derive(Component)]
pub struct TaskQueue {
    pub queue: Vec<ShipTask>,
}
