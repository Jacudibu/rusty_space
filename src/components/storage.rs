use bevy::prelude::Component;

#[derive(Component)]
pub struct Storage {
    pub capacity: u32,
    pub used: u32,
}

impl Storage {
    pub fn new(capacity: u32) -> Self {
        Self { capacity, used: 0 }
    }
}
