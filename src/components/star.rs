use crate::utils::SolarMass;
use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component)]
pub struct Star {
    pub id: Hex,
    pub mass: SolarMass,
}

impl Star {
    #[inline]
    pub fn new(id: Hex, mass: SolarMass) -> Self {
        Self { id, mass }
    }
}
