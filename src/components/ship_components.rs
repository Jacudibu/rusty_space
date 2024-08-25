//! A collection of components used for specialized ships.

use bevy::prelude::Component;

/// Required for ships to mine minerals from asteroids.
#[derive(Component)]
pub struct AsteroidMiningComponent {
    pub amount_per_second: u32,
}

/// Required for ships to harvest resources from gas giants.
#[derive(Component)]
pub struct GasHarvestingComponent {
    pub amount_per_second: u32,
}
