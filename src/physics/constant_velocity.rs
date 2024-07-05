use bevy::prelude::{Component, Vec2, Vec3};

/// Guaranteed to never change.
#[derive(Component, Default)]
pub struct ConstantVelocity {
    pub velocity: Vec3,
    pub sprite_rotation: f32,
}

impl ConstantVelocity {
    pub fn new(value: Vec2, sprite_rotation: f32) -> ConstantVelocity {
        ConstantVelocity {
            velocity: value.extend(0.0),
            sprite_rotation,
        }
    }
}
