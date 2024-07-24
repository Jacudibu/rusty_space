use bevy::math::VectorSpace;
use bevy::prelude::{Resource, Vec2};

const DEGREE_TO_RADIAN: f32 = std::f32::consts::PI / 180.0;

const TABLE_SIZE: usize = 360;

/// Contains precomputed values for cos and sin across [TABLE_SIZE], allowing us to skip computing these values multiple times per frame.
#[derive(Resource)]
pub struct OrbitTables {
    sin: [f32; TABLE_SIZE],
    cos: [f32; TABLE_SIZE],
}

impl Default for OrbitTables {
    fn default() -> Self {
        let mut result = Self {
            sin: [0.0; TABLE_SIZE],
            cos: [0.0; TABLE_SIZE],
        };

        let mut i = 0;
        while i < TABLE_SIZE {
            let angle = (i as f32) * DEGREE_TO_RADIAN;
            result.sin[i] = angle.sin();
            result.cos[i] = angle.cos();
            i += 1;
        }

        result
    }
}

impl OrbitTables {
    pub fn orbit_position_at(&self, orbit_radius: f32, fraction: f32) -> Vec2 {
        let index = (fraction * TABLE_SIZE as f32) as usize % TABLE_SIZE;
        let t = fraction * TABLE_SIZE as f32 - index as f32;

        let next_index = if index + 1 >= TABLE_SIZE {
            0
        } else {
            index + 1
        };

        Vec2 {
            x: orbit_radius * self.cos[index].lerp(self.cos[next_index], t),
            y: orbit_radius * self.sin[index].lerp(self.sin[next_index], t),
        }
    }
}
