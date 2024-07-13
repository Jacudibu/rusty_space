use crate::asteroids::{helpers, SectorWasSpawnedEvent};
use crate::components::Sector;
use crate::map_layout::MapLayout;
use crate::utils::{spawn_helpers, SimulationTime};
use crate::{constants, SpriteHandles};
use bevy::math::{ShapeSample, Vec2};
use bevy::prelude::{Circle, Commands, EventReader, Query, Res};
use rand::distributions::Distribution;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::Range;

const VELOCITY_RANDOM_RANGE: Range<f32> = 0.8..1.2;
const ROTATION_RANDOM_RANGE: Range<f32> = -0.001..0.001;

pub fn spawn_asteroids_for_new_sector(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    sprites: Res<SpriteHandles>,
    mut sector_spawns: EventReader<SectorWasSpawnedEvent>,
    mut sectors: Query<&mut Sector>,
    map_layout: Res<MapLayout>,
) {
    let now = simulation_time.now();

    for event in sector_spawns.read() {
        let mut sector = sectors.get_mut(event.sector.into()).unwrap();
        let Some(asteroid_data) = sector.asteroid_data else {
            continue;
        };

        // Technically it isn't even really necessary to further randomize positions with randomized velocity,
        // This unnatural circle shape will break up once the asteroids moved across half of the sector size.
        // TODO: We could pre-simulate that movement to make things look nicer at the start.
        let shape = Circle::new(constants::SECTOR_SIZE * 0.8);
        let seed = (sector.coordinate.x * 100000 + sector.coordinate.y) as u64;
        let position_rng = StdRng::seed_from_u64(seed);
        let mut inner_rng = StdRng::seed_from_u64(seed);

        for local_position in shape
            .interior_dist()
            .sample_iter(position_rng)
            .take(constants::ASTEROID_COUNT)
        {
            let velocity = Vec2::new(
                asteroid_data.forward_velocity.x * inner_rng.gen_range(VELOCITY_RANDOM_RANGE),
                asteroid_data.forward_velocity.y * inner_rng.gen_range(VELOCITY_RANDOM_RANGE),
            );

            let despawn_after = helpers::calculate_milliseconds_until_asteroid_leaves_hexagon(
                map_layout.hex_edge_vertices,
                local_position,
                velocity,
            );

            spawn_helpers::spawn_asteroid(
                &mut commands,
                &sprites,
                "Asteroid".to_string(),
                &mut sector,
                event.sector,
                local_position,
                velocity,
                inner_rng.gen_range(constants::ASTEROID_ORE_RANGE),
                inner_rng.gen_range(ROTATION_RANDOM_RANGE),
                now.add_milliseconds(despawn_after),
            );
        }
    }
}
