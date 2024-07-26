use crate::persistence::PersistentPlanetId;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Planet {
    pub id: PersistentPlanetId,
    // TODO: Earth masses? Maybe earth mass / 10000 to avoid floating numbers?
    pub mass: u32,
}

impl Planet {
    #[inline]
    pub fn new(id: PersistentPlanetId, mass: u32) -> Self {
        Self { id, mass }
    }
}
