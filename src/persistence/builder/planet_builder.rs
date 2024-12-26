use crate::persistence::data::v1::*;
use crate::persistence::PersistentPlanetId;
use crate::utils::{EarthMass, SolarMass};

impl SectorStarSaveData {
    pub fn new() -> Self {
        Self {
            mass: SolarMass::from_solar_mass(1, 0),
        }
    }
}

impl SectorPlanetSaveData {
    pub fn new(orbit: ConstantOrbitSaveData) -> Self {
        let id = PersistentPlanetId::next();

        Self {
            id,
            name: format!("Planet {id}"),
            kind: PlanetKindSaveData::Terrestrial,
            mass: EarthMass::from_earth_mass(1, 0),
            orbit,
        }
    }

    pub fn with_kind(mut self, kind: PlanetKindSaveData) -> Self {
        self.kind = kind;
        self
    }
}

impl ConstantOrbitSaveData {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            current_rotational_fraction: 0.0,
        }
    }

    pub fn with_current_rotational_fraction(mut self, fraction: f32) -> Self {
        self.current_rotational_fraction = fraction;
        self
    }
}
