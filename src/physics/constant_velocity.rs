use bevy::prelude::{Component, Vec2};

/// Guaranteed to never change.
#[derive(Component, Default)]
pub struct ConstantVelocity {
    pub velocity: Vec2,
    pub sprite_rotation: f32,
}

impl ConstantVelocity {
    pub fn new(velocity: Vec2, sprite_rotation: f32) -> ConstantVelocity {
        ConstantVelocity {
            velocity,
            sprite_rotation,
        }
    }
}
