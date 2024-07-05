use bevy::prelude::{Component, Vec2, Vec3};

/// Guaranteed to never change.
#[derive(Component, Default)]
pub struct ConstantVelocity {
    pub velocity: Vec3,
    pub velocity2d: Vec2,
}

impl ConstantVelocity {
    pub fn new(value: Vec2) -> ConstantVelocity {
        ConstantVelocity {
            velocity: value.extend(0.0),
            velocity2d: value,
        }
    }
}
