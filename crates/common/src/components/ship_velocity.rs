use crate::components::Engine;
use bevy::prelude::Component;

/// Fake Physics for ship movement. Has some helper methods to stir the ship depending on its engine.
#[derive(Component, Default)]
pub struct ShipVelocity {
    /// The ships' current forward velocity
    pub forward: f32,

    /// The ships' current angular velocity
    pub angular: f32,
}

impl ShipVelocity {
    /// Will immediately set forward & angular momentum to 0.
    pub fn halt_all_movement(&mut self) {
        self.forward = 0.0;
        self.angular = 0.0;
    }

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
