use crate::constants;
use crate::utils::SolarMass;

pub fn calculate_orbit_velocity(orbit_radius: f32, center_mass: SolarMass) -> f32 {
    // Instead of using the true gravitational constant, we can just adjust this value for our simulation until it "feels" right.

    ((constants::GRAVITATIONAL_CONSTANT * center_mass.inner() as f32) / orbit_radius).sqrt()
}
