use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Planet {
    pub id: PersistentPlanetId,
    pub mass: EarthMass,
}

impl Planet {
    #[inline]
    pub fn new(id: PersistentPlanetId, mass: EarthMass) -> Self {
        Self { id, mass }
    }
}

/// Marker Component for Planets with harvestable gases
#[derive(Component)]
pub struct GasGiant {}
