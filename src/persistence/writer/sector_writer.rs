use crate::persistence::data::v1::*;
use crate::simulation::physics::ConstantVelocity;
use bevy::ecs::query::QueryData;
use bevy::prelude::Query;
use common::components::celestials::Star;
use common::components::{Asteroid, Sector, SectorWithAsteroids, SectorWithCelestials};
use common::simulation_transform::SimulationTransform;
// impl AsteroidSaveData {
//     pub fn from(
//         (asteroid, transform, velocity): (&Asteroid, &SimulationTransform, &ConstantVelocity),
//     ) -> Self {
//         Self {
//             id: asteroid.id(),
//             manifest_id: asteroid.manifest_id(),
//             ore_item_id: asteroid.ore_item_id,
//             ore_current: asteroid.ore,
//             ore_max: asteroid.ore_max,
//             position: transform.translation,
//             rotation_degrees: transform.rotation.as_degrees(),
//             velocity: velocity.velocity(),
//             angular_velocity: velocity.sprite_rotation(),
//             lifetime: asteroid.despawn_timestamp,
//         }
//     }
// }
//
// impl AsteroidRespawnSaveData {
//     pub fn from_respawn(respawn: RespawningAsteroidData) -> Self {
//         Self {
//             id: respawn.id,
//             ore_max: respawn.ore_max,
//             position: respawn.local_respawn_position,
//             velocity: respawn.velocity,
//             angular_velocity: respawn.angular_velocity,
//             timestamp: respawn.timestamp,
//         }
//     }
// }
//
// impl SectorAsteroidSaveData {
//     pub fn from(
//         asteroid_component: &SectorAsteroidComponent,
//         asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
//     ) -> Self {
//         todo!();
//         // let live_asteroids = asteroid_component
//         //     .asteroids
//         //     .iter()
//         //     .map(|x| asteroid_query.get(x.entity.into()).unwrap())
//         //     .map(AsteroidSaveData::from)
//         //     .collect();
//         //
//         // let respawning_asteroids = asteroid_component
//         //     .asteroid_respawns
//         //     .iter()
//         //     .map(|x| AsteroidRespawnSaveData::from_respawn(x.0))
//         //     .collect();
//         //
//         // Self {
//         //     average_velocity: asteroid_component.average_velocity,
//         //     live_asteroids,
//         //     respawning_asteroids,
//         // }
//     }
// }
//
// impl SectorFeatureSaveData {
//     pub fn from(
//         data: SectorSaveDataQueryItem,
//         asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
//         star_query: &Query<&Star>,
//     ) -> Self {
//         SectorFeatureSaveData {
//             star: data.star.map(|x| SectorStarSaveData {
//                 mass: star_query.get(x.entity.into()).unwrap().mass,
//             }),
//             asteroids: data
//                 .asteroids
//                 .map(|x| SectorAsteroidSaveData::from(x, asteroid_query)),
//             planets: todo!(),
//         }
//     }
// }

#[derive(QueryData)]
pub struct SectorSaveDataQuery {
    sector: &'static Sector,
    star: Option<&'static SectorWithCelestials>,
    asteroids: Option<&'static SectorWithAsteroids>,
}

impl SectorSaveData {
    pub fn from(
        _data: SectorSaveDataQueryItem,
        _asteroid_query: &Query<(&Asteroid, &SimulationTransform, &ConstantVelocity)>,
        _star_query: &Query<&Star>,
    ) -> Self {
        todo!();
        // Self {
        //     coordinate: data.sector.coordinate,
        //     features: SectorFeatureSaveData::from(data, asteroid_query, star_query),
        // }
    }
}
