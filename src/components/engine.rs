use bevy::prelude::Component;

#[derive(Component)]
pub struct Engine {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            max_speed: 100.0,
            acceleration: 10.0,
            deceleration: 10.0,
            max_angular_speed: 1.0,
            angular_acceleration: 1.0,
        }
    }
}
