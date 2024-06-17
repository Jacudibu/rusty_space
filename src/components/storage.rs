use bevy::prelude::Component;

#[derive(Component)]
pub struct Storage {
    pub capacity: f32,
    pub used: f32,
}

impl Storage {
    pub fn new(capacity: f32) -> Self {
        Self {
            capacity,
            used: 0.0,
        }
    }
}
