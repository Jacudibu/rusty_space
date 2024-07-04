use crate::components::{Asteroid, Sector};
use crate::map_layout::MapLayout;
use crate::utils::{
    spawn_helpers, AsteroidEntity, CurrentSimulationTimestamp, Milliseconds, SectorEntity,
    SimulationTime, SimulationTimestamp,
};
use crate::SpriteHandles;
use bevy::prelude::{
    on_event, Alpha, App, Commands, Event, EventReader, IntoSystemConfigs, Plugin, Query, Res,
    ResMut, Resource, Sprite, Transform, Update, Vec2, Vec3, Visibility, With,
};
use bevy::time::Time;
use bevy::utils::HashSet;
use hexx::Hex;

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
    simulation_time: Res<SimulationTime>,
    sprites: Res<SpriteHandles>,
    mut sector_spawns: EventReader<SectorWasSpawnedEvent>,
    mut sectors: Query<&mut Sector>,
    map_layout: Res<MapLayout>,
) {
    // TODO glam hexx update 0.14 can skip that silly map conversion...
    let hex_edges = map_layout
        .hex_layout
        .all_edge_coordinates(Hex::ZERO)
        .map(|x| x.map(|x| Vec2::new(x.x, x.y)));

    let now = simulation_time.now();

    for event in sector_spawns.read() {
        let mut sector = sectors.get_mut(event.sector.into()).unwrap();
        let Some(asteroid_data) = sector.asteroid_data else {
            continue;
        };

        const ASTEROID_CELLS: i32 = 300; // Total = ASTEROID_CELLS² * 4
        const ASTEROID_DISTANCE: f32 = 0.5;

        for ix in 0..ASTEROID_CELLS {
            for iy in 0..ASTEROID_CELLS {
                for (x, y) in [(ix, iy), (ix, -iy), (-ix, iy), (-ix, -iy)] {
                    let local_pos =
                        Vec2::new(x as f32 * ASTEROID_DISTANCE, y as f32 * ASTEROID_DISTANCE);
                    let despawn_at = calculate_asteroid_despawn_time(
                        &now,
                        hex_edges,
                        local_pos,
                        Vec2::Y * asteroid_data.forward_velocity,
                    );

                    spawn_helpers::spawn_asteroid(
                        &mut commands,
                        &sprites,
                        format!("Asteroid [{x},{y}]"),
                        &mut sector,
                        event.sector,
                        &asteroid_data,
                        local_pos,
                        0.0,
                        despawn_at,
                    );
                }
            }
        }
    }
}

/// Intersects the two lines `(a1, a2)` and `(b1, b2)` and returns the point of intersection.
fn intersect_lines(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let denominator = (b2.y - b1.y) * (a2.x - a1.x) - (b2.x - b1.x) * (a2.y - a1.y);

    if denominator.abs() < f32::EPSILON {
        return None; // Lines are parallel
    }

    let ua = ((b2.x - b1.x) * (a1.y - b1.y) - (b2.y - b1.y) * (a1.x - b1.x)) / denominator;
    let ub = ((a2.x - a1.x) * (a1.y - b1.y) - (a2.y - a1.y) * (a1.x - b1.x)) / denominator;

    if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
        let x = a1.x + ua * (a2.x - a1.x);
        let y = a1.y + ua * (a2.y - a1.y);
        return Some(Vec2 { x, y });
    }

    None
}

/// ## Panics
/// If the point is not within the hexagon or velocity is 0.
fn calculate_asteroid_despawn_time(
    now: &CurrentSimulationTimestamp,
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_spawn_position: Vec2,
    velocity: Vec2,
) -> SimulationTimestamp {
    debug_assert!(velocity.length_squared() > 0.0);
    let mut time = -1.0;

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = intersect_lines(
            local_spawn_position,
            local_spawn_position + velocity * 100000000.0, // Gotta make sure to offset it nicely
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance(local_spawn_position);
            time = distance / velocity.length();
            break;
        }
    }

    now.add_seconds(time as u64)
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

/// Needs to run before [spawn_asteroids] in order to ensure no new asteroids are spawned which aren't yet synced.
/// Technically this doesn't need to run every frame, given the super slow speed of asteroids.
pub fn make_asteroids_disappear_when_they_leave_sector(
    mut fading_asteroids: ResMut<FadingAsteroids>,
    mut sector: Query<&mut Sector>,
    simulation_time: Res<SimulationTime>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        while let Some(next) = sector.asteroids.peek() {
            if now.has_not_passed(next.despawn_at) {
                break;
            }

            let ded_asteroid = sector.asteroids.pop().unwrap();
            fading_asteroids.asteroids.insert(ded_asteroid.entity);
        }
    }
}

/// Fades asteroid alpha values to 0 before finally turning their visibility off.
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
