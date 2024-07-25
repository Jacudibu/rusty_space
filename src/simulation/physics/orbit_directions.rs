use bevy::math::VectorSpace;
use bevy::prelude::{Dir2, Resource, Vec2};

const DEGREE_TO_RADIAN: f32 = std::f32::consts::PI / 180.0;

const AMOUNT: usize = 360;

/// Contains [`AMOUNT`] precomputed values pointing at the outline of a uniform sphere, allowing us to skip computing these values multiple times per frame.
#[derive(Resource)]
pub struct OrbitDirections {
    directions: [Dir2; AMOUNT],
}

impl Default for OrbitDirections {
    fn default() -> Self {
        let mut result = Self {
            directions: [Dir2::Y; AMOUNT],
        };

        for i in 0..AMOUNT {
            let angle = (i as f32) * DEGREE_TO_RADIAN;

            result.directions[i] = Dir2::new_unchecked(Vec2 {
                x: angle.sin(),
                y: angle.cos(),
            });
        }

        result
    }
}

impl OrbitDirections {
    pub fn orbit_position_at(&self, orbit_radius: f32, fraction: f32) -> Vec2 {
        let index = (fraction * AMOUNT as f32) as usize % AMOUNT;
        let t = fraction * AMOUNT as f32 - index as f32;

        let next_index = if index + 1 >= AMOUNT { 0 } else { index + 1 };

        let a = self.directions[index];
        let b = self.directions[next_index];

        Vec2 {
            x: orbit_radius * a.x.lerp(b.x, t),
            y: orbit_radius * a.y.lerp(b.y, t),
        }
    }
}
