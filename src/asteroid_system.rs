use crate::components::{Asteroid, Sector};
use crate::map_layout::MapLayout;
use crate::utils::{spawn_helpers, AsteroidEntity, SectorEntity};
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    error, on_event, Alpha, App, Commands, Event, EventReader, IntoSystemConfigs, Plugin, Query,
    Res, ResMut, Resource, Sprite, Transform, Update, Vec2, Vec3, Visibility, With,
};
use bevy::time::Time;
use bevy::utils::HashSet;

/// ### General Idea
/// Every Sector may have asteroids inside it, defined by its [SectorAsteroidData].
/// Every Sector keeps track of its "alive" Asteroids inside their `asteroids` variable.
/// There is a fixed amount of asteroids within each sector.
/// Once an asteroid is mined or floats outside the sector borders, it gets removed from the sectors
/// `asteroid` variable and its visibility is turned off.
/// Another system keeps track of an ordered Queue with all "dead" asteroids, resetting their position
/// and visibility once a set [SimulationTimestamp] has been reached.
pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FadingAsteroids>()
            .add_event::<SectorWasSpawnedEvent>()
            .add_systems(
                Update,
                (
                    spawn_asteroids.run_if(on_event::<SectorWasSpawnedEvent>()),
                    make_asteroids_disappear_when_they_leave_sector.before(spawn_asteroids),
                    fade_asteroids,
                ),
            );
    }
}
#[derive(Event)]
pub struct SectorWasSpawnedEvent {
    pub(crate) sector: SectorEntity,
}

pub fn spawn_asteroids(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_spawns: EventReader<SectorWasSpawnedEvent>,
    mut sectors: Query<&mut Sector>,
) {
    for event in sector_spawns.read() {
        let mut sector = sectors.get_mut(event.sector.into()).unwrap();
        let Some(asteroid_data) = sector.asteroid_data else {
            continue;
        };

        const ASTEROID_CELLS: i32 = 400; // Total = ASTEROID_CELLS² * 4
        const ASTEROID_DISTANCE: f32 = 0.5;

        for ix in 0..ASTEROID_CELLS {
            for iy in 0..ASTEROID_CELLS {
                for (x, y) in [(ix, iy), (ix, -iy), (-ix, iy), (-ix, -iy)] {
                    let local_pos =
                        Vec2::new(x as f32 * ASTEROID_DISTANCE, y as f32 * ASTEROID_DISTANCE);
                    spawn_helpers::spawn_asteroid(
                        &mut commands,
                        &sprites,
                        format!("Asteroid [{x},{y}]"),
                        &mut sector,
                        event.sector,
                        &asteroid_data,
                        local_pos,
                        0.0,
                    );
                }
            }
        }
    }
}

fn is_point_within_hexagon(point: Vec3, edges: [[hexx::Vec2; 2]; 6]) -> bool {
    let mut intersections = 0;
    for [a, b] in edges {
        let is_between_y = (a.y > point.y) != (b.y > point.y);
        if is_between_y && (point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x) {
            intersections += 1;
        }
    }

    intersections == 1
}

#[derive(Resource, Default)]
pub struct FadingAsteroids {
    pub asteroids: HashSet<AsteroidEntity>,
}

/* Performance Ideas
 Asteroids are supposed to move very slowly, so this really doesn't need to be checked every frame for every asteroid.
 Keep a resource with a VecDequeue of all Sectors with asteroids, and only test one sector per frame.
*/

/// Needs to run before [spawn_asteroids] in order to ensure no new asteroids are spawned which aren't yet synced.
pub fn make_asteroids_disappear_when_they_leave_sector(
    asteroids: Query<&Transform, With<Asteroid>>,
    mut fading_asteroids: ResMut<FadingAsteroids>,
    mut sector: Query<&mut Sector>,
    map_layout: Res<MapLayout>,
) {
    for mut sector in sector.iter_mut() {
        let edges = map_layout
            .hex_layout
            .all_edge_coordinates(sector.coordinate);

        let mut removals = Vec::new();
        for asteroid_entity in sector.asteroids.iter() {
            let transform = asteroids.get(asteroid_entity.into()).unwrap();

            if is_point_within_hexagon(transform.translation, edges) {
                continue;
            }

            removals.push(*asteroid_entity);
        }

        for x in removals {
            sector.remove_asteroid_in_place(x);
            fading_asteroids.asteroids.insert(x);
        }
    }
}

/// Fades asteroid alpha values to 0, before turning their visibility off.
pub fn fade_asteroids(
    time: Res<Time>,
    mut fading_asteroids: ResMut<FadingAsteroids>,
    mut asteroid_query: Query<(&mut Sprite, &mut Visibility), With<Asteroid>>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let (mut sprite, mut visibility) = asteroid_query.get_mut(entity.into()).unwrap();

        let alpha = sprite.color.alpha() - time.delta_seconds();
        if alpha > 0.0 {
            sprite.color.set_alpha(alpha);
        } else {
            sprite.color.set_alpha(0.0);
            *visibility = Visibility::Hidden;
            removals.insert(*entity);
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}
