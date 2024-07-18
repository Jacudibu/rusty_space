use crate::components::{Asteroid, Sector, SectorAsteroidData};
use crate::persistence::data::v1::*;
use crate::persistence::ComponentWithPersistentId;
use crate::physics::ConstantVelocity;
use crate::simulation_transform::SimulationTransform;
use bevy::prelude::Query;

impl AsteroidSaveData {
    pub fn from(
        (asteroid, transform, velocity): (&Asteroid, &SimulationTransform, &ConstantVelocity),
    ) -> Self {
        Self {
            id: asteroid.id(),
            ore_current: asteroid.ore,
            ore_max: asteroid.ore_max,
            position: transform.translation,
            rotation_degrees: transform.rotation.as_degrees(),
            velocity: velocity.velocity,
            angular_velocity: velocity.sprite_rotation,
            lifetime: asteroid.state.timestamp(),
        }
    }
}

impl SectorAsteroidSaveData {
    pub fn from(data: &SectorAsteroidData) -> Self {
        Self {
            average_velocity: data.average_velocity,
        }
    }
}

impl SectorSaveData {
    pub fn from(
        sector: &Sector,
        asteroids: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
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
            asteroid_data: sector
                .asteroid_data
                .as_ref()
                .map(SectorAsteroidSaveData::from),
            live_asteroids,
            respawning_asteroids,
        }
    }
}
