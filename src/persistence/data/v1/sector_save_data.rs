use crate::game_data::{AsteroidDataId, ItemId};
use crate::persistence::{PersistentAsteroidId, SectorPlanetSaveData};
use crate::simulation::prelude::SimulationTimestamp;
use crate::utils::SolarMass;
use bevy::math::Vec2;
use hexx::Hex;
use serde::{Deserialize, Serialize};

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
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorStarSaveData {
    pub mass: SolarMass,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorFeatureSaveData {
    pub star: Option<SectorStarSaveData>,
    pub asteroids: Option<SectorAsteroidSaveData>,
    pub planets: Option<Vec<SectorPlanetSaveData>>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub features: SectorFeatureSaveData,
}
