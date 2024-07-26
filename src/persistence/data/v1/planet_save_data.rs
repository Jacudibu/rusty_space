use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorPlanetSaveData {
    pub id: PersistentPlanetId,
    pub name: String,
    pub mass: EarthMass,
    pub orbit: ConstantOrbitSaveData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct ConstantOrbitSaveData {
    pub current_rotational_fraction: f32,
    pub radius: f32,
}
