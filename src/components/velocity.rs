use crate::components::engine::Engine;
use bevy::prelude::Component;

/// Fake Physics!
#[derive(Component, Default)]
pub struct Velocity {
    pub forward: f32,
    pub angular: f32,
}

impl Velocity {
    pub fn accelerate(&mut self, engine: &Engine, delta_seconds: f32) {
        self.forward += engine.acceleration * delta_seconds;
        if self.forward > engine.max_speed {
            self.forward = engine.max_speed;
        }
    }

    pub fn decelerate(&mut self, engine: &Engine, delta_seconds: f32) {
        self.forward -= engine.deceleration * delta_seconds;
        if self.forward < 0.0 {
            self.forward = 0.0;
        }
    }

    pub fn turn_right(&mut self, engine: &Engine, delta_seconds: f32) {
        self.angular -= engine.angular_acceleration * delta_seconds;
        if self.angular < -engine.max_angular_speed {
            self.angular = -engine.max_angular_speed;
        }
    }

    pub fn turn_left(&mut self, engine: &Engine, delta_seconds: f32) {
        self.angular += engine.angular_acceleration * delta_seconds;
        if self.angular > engine.max_angular_speed {
            self.angular = engine.max_angular_speed;
        }
    }
}
