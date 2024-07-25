use crate::components::{Asteroid, Sector, SectorAsteroidComponent, SectorFeature};
use crate::persistence::data::v1::*;
use crate::persistence::ComponentWithPersistentId;
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::transform::simulation_transform::SimulationTransform;
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

impl SectorAsteroidFeatureSaveData {
    pub fn from(
        feature: &SectorAsteroidComponent,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
    ) -> Self {
        let live_asteroids = feature
            .asteroids
            .iter()
            .map(|x| asteroid_query.get(x.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();
        let respawning_asteroids = feature
            .asteroid_respawns
            .iter()
            .map(|x| asteroid_query.get(x.0.entity.into()).unwrap())
            .map(AsteroidSaveData::from)
            .collect();

        Self {
            average_velocity: feature.asteroid_data.average_velocity,
            live_asteroids,
            respawning_asteroids,
        }
    }
}

impl SectorFeatureSaveData {
    pub fn from(
        feature: &SectorFeature,
        asteroid_component: Option<&SectorAsteroidComponent>,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
    ) -> Self {
        match feature {
            SectorFeature::Void => SectorFeatureSaveData::Void,
            SectorFeature::Star => SectorFeatureSaveData::Star,
            SectorFeature::AsteroidCloud => SectorFeatureSaveData::AsteroidCloud(
                SectorAsteroidFeatureSaveData::from(asteroid_component.unwrap(), asteroid_query),
            ),
        }
    }
}

impl SectorSaveData {
    pub fn from(
        sector: &Sector,
        asteroid_component: Option<&SectorAsteroidComponent>,
        asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
    ) -> Self {
        Self {
            coordinate: sector.coordinate,
            feature: SectorFeatureSaveData::from(
                &sector.feature,
                asteroid_component,
                asteroid_query,
            ),
        }
    }
}
