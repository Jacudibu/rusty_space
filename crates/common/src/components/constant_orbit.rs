use crate::constants;
use crate::types::celestial_mass::CelestialMass;
use crate::types::polar_coordinates::PolarCoordinates;
use bevy::prelude::Component;

/// A component used to simulate a celestial body which permanently orbits some point in space in a fixed, circular shape
#[derive(Component)]
pub struct ConstantOrbit {
    /// The current position of this entity represented as [PolarCoordinates].
    pub polar_coordinates: PolarCoordinates,
    /// The velocity at which we are moving forward.
    pub velocity: f32,
}

impl ConstantOrbit {
    #[inline]
    pub fn new(polar_coordinates: PolarCoordinates, center_mass: &CelestialMass) -> Self {
        Self {
            velocity: Self::calculate_orbit_velocity(
                polar_coordinates.radial_distance,
                center_mass,
            ),
            polar_coordinates,
        }
    }

    fn calculate_orbit_velocity(orbit_radius: f32, center_mass: &CelestialMass) -> f32 {
        // Instead of using the true gravitational constant, we can just adjust this value for our simulation until it "feels" right.
        let center_mass = match center_mass {
            CelestialMass::SolarMass(mass) => mass.inner() as f32,
            CelestialMass::EarthMass(_mass) => {
                todo!("Super slow!")
            }
        };

        ((constants::GRAVITATIONAL_CONSTANT * center_mass) / orbit_radius).sqrt()
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
