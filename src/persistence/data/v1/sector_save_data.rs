use crate::game_data::{AsteroidDataId, ItemId};
use crate::persistence::{PersistentAsteroidId, PersistentCelestialId};
use crate::simulation::prelude::SimulationTimestamp;
use crate::utils::CelestialMass;
use bevy::math::Vec2;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct AsteroidSaveData {
    pub id: PersistentAsteroidId,
    pub manifest_id: AsteroidDataId,
    pub ore_item_id: ItemId,
    pub ore_current: u32,
    pub ore_max: u32,
    pub position: Vec2,
    pub rotation_degrees: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub lifetime: SimulationTimestamp,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct AsteroidRespawnSaveData {
    pub id: PersistentAsteroidId,
    pub ore_max: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub timestamp: SimulationTimestamp,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorAsteroidSaveData {
    pub average_velocity: Vec2,
    pub asteroid_materials: Vec<ItemId>,
    pub live_asteroids: Vec<AsteroidSaveData>,
    pub respawning_asteroids: Vec<AsteroidRespawnSaveData>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorCelestialSaveData {
    pub id: PersistentCelestialId,
    pub kind: CelestialKindSaveData,
    pub name: String,
    pub mass: CelestialMass,
    pub local_position: Vec2,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum CelestialKindSaveData {
    Star,
    Terrestrial,
    GasGiant { resources: Vec<ItemId> },
}

impl Display for CelestialKindSaveData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CelestialKindSaveData::Star => f.write_str("Star"),
            CelestialKindSaveData::Terrestrial => f.write_str("Planet"),
            CelestialKindSaveData::GasGiant { .. } => f.write_str("Gas Giant"),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorCelestialsSaveData {
    pub center_mass: CelestialMass,
    pub celestials: Vec<SectorCelestialSaveData>,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorFeatureSaveData {
    pub asteroids: Option<SectorAsteroidSaveData>,
    pub celestials: Option<SectorCelestialsSaveData>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub features: SectorFeatureSaveData,
}
