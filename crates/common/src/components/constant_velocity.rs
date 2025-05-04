use bevy::prelude::{Component, Vec2};

/// An entity with a [ConstantVelocity] applied to it will always move in the specified position.
#[derive(Component, Default)]
#[component(immutable)]
pub struct ConstantVelocity {
    /// constant velocity in global space
    velocity: Vec2,

    /// constant rotational velocity
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
