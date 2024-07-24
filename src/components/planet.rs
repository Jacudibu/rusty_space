use crate::persistence::PersistentPlanetId;
use bevy::prelude::{Component, Vec2};

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

/// Used to simulate a celestial body that's permanently orbiting some point in space in a fixed, circular shape
#[derive(Component)]
pub struct ConstantOrbit {
    pub around: Vec2,
    pub current_orbit_angle: f32,
    pub orbit_distance: f32,
    pub velocity: f32,
}

impl ConstantOrbit {
    pub fn new(around: Vec2, orbit_angle: f32, orbit_distance: f32, velocity: f32) -> Self {
        Self {
            around,
            current_orbit_angle: orbit_angle,
            orbit_distance,
            velocity,
        }
    }
}
