use crate::constants;
use crate::utils::SolarMass;
use crate::utils::polar_coordinates::PolarCoordinates;
use bevy::prelude::Component;

/// Used to simulate a celestial body which permanently orbits some point in space in a fixed, circular shape
#[derive(Component)]
pub struct ConstantOrbit {
    /// The current position of this entity represented as [PolarCoordinates].
    pub polar_coordinates: PolarCoordinates,
    /// The velocity at which we are moving forward.
    pub velocity: f32,
}

impl ConstantOrbit {
    #[inline]
    pub fn new(polar_coordinates: PolarCoordinates, center_mass: &SolarMass) -> Self {
        Self {
            velocity: Self::calculate_orbit_velocity(polar_coordinates.radial, center_mass),
            polar_coordinates,
        }
    }

    fn calculate_orbit_velocity(orbit_radius: f32, center_mass: &SolarMass) -> f32 {
        // Instead of using the true gravitational constant, we can just adjust this value for our simulation until it "feels" right.
        ((constants::GRAVITATIONAL_CONSTANT * center_mass.inner() as f32) / orbit_radius).sqrt()
    }

    /// Moves this celestial body forward in its rotational path.
    #[inline]
    pub fn advance(&mut self, delta: f32) {
        self.polar_coordinates.angle += self.velocity * delta;
        if self.polar_coordinates.angle >= 360.0 {
            self.polar_coordinates.angle -= 360.0;
        }
    }
}
