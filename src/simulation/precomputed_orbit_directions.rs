use crate::utils::polar_coordinates::PolarCoordinates;
use bevy::math::VectorSpace;
use bevy::prelude::{Dir2, Resource, Vec2};

/// Contains 360 precomputed values pointing at the outline of a uniform sphere, allowing us to skip computing these values multiple times per frame.
#[derive(Resource)]
pub struct PrecomputedOrbitDirections {
    directions: [Dir2; 360],
}

impl Default for PrecomputedOrbitDirections {
    fn default() -> Self {
        let mut result = Self {
            directions: [Dir2::Y; 360],
        };

        for i in 0..360 {
            let angle = (i as f32).to_radians();

            result.directions[i] = Dir2::new_unchecked(Vec2 {
                x: angle.cos(),
                y: angle.sin(),
            });
        }

        result
    }
}

impl PrecomputedOrbitDirections {
    /// Converts the given position represented in [PolarCoordinates] as a local position in Cartesian Coordinates.
    pub fn convert_polar_to_local_cartesian(&self, pos: &PolarCoordinates) -> Vec2 {
        let index = pos.angle as usize % 360;
        let t = pos.angle - index as f32;

        let next_index = if index + 1 >= 360 { 0 } else { index + 1 };

        let a = self.directions[index];
        let b = self.directions[next_index];

        Vec2 {
            x: pos.radial * a.x.lerp(b.x, t),
            y: pos.radial * a.y.lerp(b.y, t),
        }
    }
}
