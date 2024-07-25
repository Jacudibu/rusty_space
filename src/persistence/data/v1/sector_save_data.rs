use crate::persistence::PersistentAsteroidId;
use crate::simulation::prelude::SimulationTimestamp;
use bevy::math::Vec2;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct AsteroidSaveData {
    pub id: PersistentAsteroidId,
    pub ore_current: u32,
    pub ore_max: u32,
    pub position: Vec2,
    pub rotation_degrees: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub lifetime: SimulationTimestamp,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct SectorAsteroidFeatureSaveData {
    pub average_velocity: Vec2,
    pub live_asteroids: Vec<AsteroidSaveData>,
    pub respawning_asteroids: Vec<AsteroidSaveData>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub enum SectorFeatureSaveData {
    Void,
    Star,
    AsteroidCloud(SectorAsteroidFeatureSaveData),
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub feature: SectorFeatureSaveData,
}
