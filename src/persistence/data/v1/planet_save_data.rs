use crate::persistence::PersistentPlanetId;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct PlanetSaveData {
    pub id: PersistentPlanetId,
    pub name: String,
    pub sector: Hex,
    pub mass: u32,
    pub orbit: ConstantOrbitSaveData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct ConstantOrbitSaveData {
    pub current_rotational_fraction: f32,
    pub radius: f32,
}
