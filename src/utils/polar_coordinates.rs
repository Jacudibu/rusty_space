use bevy::prelude::Vec2;

/// Represents a position in PolarCoordinates.
pub struct PolarCoordinates {
    /// The radial coordinate r, indicating our distance from the pole.
    pub radial_distance: f32,
    /// The polar angle. 0 when pointing to the right, increasing counterclockwise up to 360 after going full circle.
    pub angle: f32,
}

impl PolarCoordinates {
    /// Converts a regular position represented as a [Vec2] into [PolarCoordinates].
    pub fn from_cartesian(pos: &Vec2) -> Self {
        let mut angle_in_radians = pos.y.atan2(pos.x);

        // Ensure we are in [0,2pi[ rather than [-pi,pi[
        if angle_in_radians < 0.0 {
            angle_in_radians += std::f32::consts::TAU;
        }

        Self {
            radial_distance: (pos.x * pos.x + pos.y * pos.y).sqrt(),
            angle: angle_in_radians.to_degrees(),
        }
    }

    pub fn to_cartesian(&self) -> Vec2 {
        Vec2 {
            x: self.radial_distance * self.angle.cos(),
            y: self.radial_distance * self.angle.sin(),
        }
    }
}
