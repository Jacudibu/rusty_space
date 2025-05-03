use crate::game_data::ItemId;
use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;
use bevy::prelude::Component;

/// Overarching marker component for Planet Entities.
#[derive(Component)]
#[component(immutable)]
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

/// Marker Component for [Planet]s with harvestable gases
#[derive(Component)]
#[component(immutable)]
pub struct GasGiant {
    pub resources: Vec<ItemId>,
}
