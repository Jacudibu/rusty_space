use bevy::prelude::{Deref, DerefMut, Vec2};
use common::types::celestial_mass::{CelestialMass, EarthMass};
use common::types::persistent_entity_id::PersistentCelestialId;
use persistence::data::{
    CelestialKindSaveData, IndividualSectorCelestialSaveData, SectorCelestialsSaveData,
};

pub struct CelestialBuilder {
    pub(crate) data: SectorCelestialsSaveData,
}

#[derive(Deref, DerefMut)]
pub struct SectorCelestialBuilder {
    pub(crate) data: IndividualSectorCelestialSaveData,
}

impl CelestialBuilder {
    pub fn new(center_celestial: IndividualSectorCelestialSaveData) -> Self {
        Self {
            data: SectorCelestialsSaveData {
                center_mass: center_celestial.mass,
                celestials: vec![center_celestial],
            },
        }
    }
}

impl SectorCelestialBuilder {
    pub fn new(kind: CelestialKindSaveData, local_position: Vec2) -> Self {
        let id = PersistentCelestialId::next();

        Self {
            data: IndividualSectorCelestialSaveData {
                id,
                name: format!("{kind} {id}"),
                kind,
                mass: CelestialMass::EarthMass(EarthMass::from_earth_mass(1, 0)),
                local_position,
            },
        }
    }

    pub fn with_mass(mut self, mass: CelestialMass) -> Self {
        self.mass = mass;
        self
    }
}
