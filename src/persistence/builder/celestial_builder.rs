use crate::persistence::PersistentCelestialId;
use crate::persistence::data::v1::*;
use crate::utils::{CelestialMass, EarthMass};
use bevy::prelude::Vec2;

impl SectorCelestialsSaveData {
    pub fn new(center_celestial: SectorCelestialSaveData) -> Self {
        Self {
            center_mass: center_celestial.mass,
            celestials: vec![center_celestial],
        }
    }
}

impl SectorCelestialSaveData {
    pub fn new(kind: CelestialKindSaveData, local_position: Vec2) -> Self {
        let id = PersistentCelestialId::next();

        Self {
            id,
            name: format!("{kind} {id}"),
            kind,
            mass: CelestialMass::EarthMass(EarthMass::from_earth_mass(1, 0)),
            local_position,
        }
    }

    pub fn with_mass(mut self, mass: CelestialMass) -> Self {
        self.mass = mass;
        self
    }
}
