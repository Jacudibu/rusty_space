pub fn calculate_orbit_velocity(orbit_radius: f32, center_mass: f32) -> f32 {
    // TODO: Instead of using a realistic gravitational constant, we can just adjust this value for our simulation until it "feels" right, that's why this value is bogus
    const GRAVITATIONAL_CONSTANT: f32 = 0.000067;

    ((GRAVITATIONAL_CONSTANT * center_mass) / orbit_radius).sqrt()
}
