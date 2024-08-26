use crate::session_data::ship_configs::EngineStats;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Engine {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}

impl From<&EngineStats> for Engine {
    fn from(value: &EngineStats) -> Self {
        Self {
            max_speed: value.max_speed,
            acceleration: value.acceleration,
            deceleration: value.deceleration,
            max_angular_speed: value.max_angular_speed,
            angular_acceleration: value.angular_acceleration,
        }
    }
}
