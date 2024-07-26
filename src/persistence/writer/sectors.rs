use crate::components::{Asteroid, Sector, SectorAsteroidComponent, SectorStarComponent, Star};
use crate::persistence::data::v1::*;
use crate::persistence::ComponentWithPersistentId;
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use bevy::ecs::query::QueryData;
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
            velocity: velocity.velocity(),
            angular_velocity: velocity.sprite_rotation(),
            lifetime: asteroid.state.timestamp(),
        }
    }
}

impl SectorAsteroidSaveData {
    pub fn from(
        asteroid_component: &SectorAsteroidComponent,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
    ) -> Self {
        let live_asteroids = asteroid_component
            .asteroids
            .iter()
            .map(|x| asteroid_query.get(x.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();
        let respawning_asteroids = asteroid_component
            .asteroid_respawns
            .iter()
            .map(|x| asteroid_query.get(x.0.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();

        Self {
            average_velocity: asteroid_component.average_velocity,
            live_asteroids,
            respawning_asteroids,
        }
    }
}

impl SectorFeatureSaveData {
    pub fn from(
        data: SectorSaveDataQueryItem,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
        star_query: &Query<&Star>,
    ) -> Self {
        SectorFeatureSaveData {
            star: data.star.map(|x| SectorStarSaveData {
                mass: star_query.get(x.entity.into()).unwrap().mass,
            }),
            asteroids: data
                .asteroids
                .map(|x| SectorAsteroidSaveData::from(x, asteroid_query)),
            planets: todo!(),
        }
    }
}

#[derive(QueryData)]
pub struct SectorSaveDataQuery {
    sector: &'static Sector,
    star: Option<&'static SectorStarComponent>,
    asteroids: Option<&'static SectorAsteroidComponent>,
}

impl SectorSaveData {
    pub fn from(
        data: SectorSaveDataQueryItem,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
        star_query: &Query<&Star>,
    ) -> Self {
        Self {
            coordinate: data.sector.coordinate,
            features: SectorFeatureSaveData::from(data, asteroid_query, star_query),
        }
    }
}
