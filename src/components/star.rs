use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component)]
pub struct Star {
    pub id: Hex,
    // TODO: Solar masses? Maybe solar mass / 100 to avoid floating numbers?
    pub mass: u32,
}

impl Star {
    #[inline]
    pub fn new(id: Hex, mass: u32) -> Self {
        Self { id, mass }
    }
}
