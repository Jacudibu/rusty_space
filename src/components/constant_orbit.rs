use bevy::prelude::Component;

/// Used to simulate a celestial body which permanently orbits some point in space in a fixed, circular shape
#[derive(Component)]
pub struct ConstantOrbit {
    /// depicts the current position on the circle in [0,1[
    pub rotational_fraction: f32,
    /// The distance from the center point we are orbiting.
    pub radius: f32,
    /// The velocity at which we are moving forward.
    pub velocity: f32,
}

pub struct PolarCoordinates {
    /// The radial coordinate r, indicating our distance from the center.
    radial: f32,
    /// The polar angle, ranging from [0,360[ - 0 when pointing to the right, increasing counterclockwise
    angle: f32,
}

impl ConstantOrbit {
    #[inline]
    pub fn new(orbit_angle: f32, orbit_distance: f32, velocity: f32) -> Self {
        Self {
            rotational_fraction: orbit_angle,
            radius: orbit_distance,
            velocity,
        }
    }

    /// Moves this celestial body forward on its rotational path.
    #[inline]
    pub fn advance(&mut self, delta: f32) {
        self.rotational_fraction += self.velocity * delta;
        if self.rotational_fraction >= 1.0 {
            self.rotational_fraction -= 1.0;
        }
    }
}
