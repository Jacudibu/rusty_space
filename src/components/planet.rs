use crate::game_data::ItemId;
use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;
use bevy::prelude::Component;

#[derive(Component)]
pub struct PlanetComponent {
    pub id: PersistentPlanetId,
    pub mass: EarthMass,
}

impl PlanetComponent {
    #[inline]
    pub fn new(id: PersistentPlanetId, mass: EarthMass) -> Self {
        Self { id, mass }
    }
}

/// Marker Component for Planets with harvestable gases
#[derive(Component)]
pub struct GasGiant {
    pub resources: Vec<ItemId>,
}
