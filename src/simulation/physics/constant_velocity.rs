use bevy::prelude::{Component, Vec2};

/// Guaranteed to never change.
#[derive(Component, Default)]
pub struct ConstantVelocity {
    velocity: Vec2,
    sprite_rotation: f32,
}

impl ConstantVelocity {
    #[inline]
    pub fn new(velocity: Vec2, sprite_rotation: f32) -> ConstantVelocity {
        ConstantVelocity {
            velocity,
            sprite_rotation,
        }
    }

    #[inline]
    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    #[inline]
    pub fn sprite_rotation(&self) -> f32 {
        self.sprite_rotation
    }
}
