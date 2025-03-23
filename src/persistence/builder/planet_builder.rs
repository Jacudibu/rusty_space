use crate::persistence::PersistentPlanetId;
use crate::persistence::data::v1::*;
use crate::utils::{EarthMass, SolarMass};
use bevy::prelude::Vec2;

impl SectorStarSaveData {
    pub fn new() -> Self {
        Self {
            mass: SolarMass::from_solar_mass(1, 0),
        }
    }
}

impl SectorPlanetSaveData {
    pub fn new(local_position: Vec2) -> Self {
        let id = PersistentPlanetId::next();

        Self {
            id,
            name: format!("Planet {id}"),
            kind: PlanetKindSaveData::Terrestrial,
            mass: EarthMass::from_earth_mass(1, 0),
            local_position,
        }
    }

    pub fn with_kind(mut self, kind: PlanetKindSaveData) -> Self {
        self.kind = kind;
        self
    }
}
