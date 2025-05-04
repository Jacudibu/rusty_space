//! A collection of components used for specialized ships.

use bevy::prelude::Component;

/// A [Ship] with this component may harvest minerals from asteroids.
#[derive(Component)]
#[component(immutable)]
pub struct AsteroidMiner {
    pub amount_per_second: u32,
}

/// A [Ship] with this component may harvest resources from gas giants.
#[derive(Component)]
pub struct GasHarvester {
    pub amount_per_second: u32,
}
