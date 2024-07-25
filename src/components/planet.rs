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

/// Used to simulate a celestial body that's permanently orbiting some point in space in a fixed, circular shape
#[derive(Component)]
pub struct ConstantOrbit {
    /// depicts the current position on the circle in [0,1[
    pub rotational_fraction: f32,
    pub radius: f32,
    pub velocity: f32,
}

impl ConstantOrbit {
    pub fn new(orbit_angle: f32, orbit_distance: f32, velocity: f32) -> Self {
        Self {
            rotational_fraction: orbit_angle,
            radius: orbit_distance,
            velocity,
        }
    }

    #[inline]
    pub fn advance(&mut self, delta: f32) {
        self.rotational_fraction += self.velocity * delta;
        if self.rotational_fraction >= 1.0 {
            self.rotational_fraction -= 1.0;
        }
    }
}
