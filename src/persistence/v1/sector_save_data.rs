use crate::components::{Asteroid, Sector, SectorAsteroidData};
use crate::physics::ConstantVelocity;
use crate::utils::SimulationTimestamp;
use bevy::math::{EulerRot, Vec2};
use bevy::prelude::{Query, Transform};
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AsteroidSaveData {
    pub ore_current: u32,
    pub ore_max: u32,
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub lifetime: SimulationTimestamp,
}

impl AsteroidSaveData {
    pub fn from(
        (asteroid, transform, velocity): (&Asteroid, &Transform, &ConstantVelocity),
    ) -> Self {
        Self {
            ore_current: asteroid.ore,
            ore_max: asteroid.ore_max,
            position: transform.translation.truncate(),
            rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
            velocity: velocity.velocity.truncate(),
            angular_velocity: velocity.sprite_rotation,
            lifetime: asteroid.state.timestamp(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SectorAsteroidSaveData {
    pub average_velocity: Vec2,
}

impl SectorAsteroidSaveData {
    pub fn from(data: SectorAsteroidData) -> Self {
        Self {
            average_velocity: data.average_velocity,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub asteroid_data: Option<SectorAsteroidSaveData>,
    pub live_asteroids: Vec<AsteroidSaveData>,
    pub respawning_asteroids: Vec<AsteroidSaveData>,
}

impl SectorSaveData {
    pub fn from(
        sector: &Sector,
        asteroids: &Query<(&Asteroid, &Transform, &ConstantVelocity)>,
    ) -> Self {
        let live_asteroids = sector
            .asteroids
            .iter()
            .map(|x| asteroids.get(x.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();

        let respawning_asteroids = sector
            .asteroid_respawns
            .iter()
            .map(|x| asteroids.get(x.0.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();

        Self {
            coordinate: sector.coordinate,
            asteroid_data: sector.asteroid_data.map(SectorAsteroidSaveData::from),
            live_asteroids,
            respawning_asteroids,
        }
    }
}
