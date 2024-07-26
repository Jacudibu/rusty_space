use crate::persistence::data::v1::*;
use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;

impl SectorPlanetSaveData {
    pub fn new(name: String, mass: EarthMass, orbit: ConstantOrbitSaveData) -> Self {
        Self {
            id: PersistentPlanetId::next(),
            name,
            mass,
            orbit,
        }
    }
}
