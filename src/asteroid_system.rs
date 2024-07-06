use crate::components::{Asteroid, Sector};
use crate::map_layout::MapLayout;
use crate::physics::ConstantVelocity;
use crate::utils::{spawn_helpers, AsteroidEntity, Milliseconds, SectorEntity, SimulationTime};
use crate::{constants, SpriteHandles};
use bevy::prelude::{
    on_event, Alpha, App, Commands, Event, EventReader, Gizmos, IntoSystemConfigs, Plugin, Query,
    Res, ResMut, Resource, Sprite, Transform, Update, Vec2, Visibility, With,
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
        app.init_resource::<FadingAsteroidsOut>()
            .init_resource::<FadingAsteroidsIn>()
            .add_event::<SectorWasSpawnedEvent>()
            .add_systems(
                Update,
                (
                    spawn_asteroids.run_if(on_event::<SectorWasSpawnedEvent>()),
                    make_asteroids_disappear_when_they_leave_sector.before(spawn_asteroids),
                    respawn_asteroids,
                    fade_asteroids_out,
                    fade_asteroids_in,
                    //draw_asteroid_debug_gizmos,
                ),
            );
    }
}
#[derive(Event)]
pub struct SectorWasSpawnedEvent {
    pub(crate) sector: SectorEntity,
}

fn draw_asteroid_debug_gizmos(mut gizmos: Gizmos, asteroids: Query<&Transform, With<Asteroid>>) {
    for transform in asteroids.iter() {
        gizmos.circle_2d(
            transform.translation.truncate(),
            4.0,
            bevy::color::palettes::css::DARK_ORCHID,
        );
    }
}

pub fn spawn_asteroids(
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

        const ASTEROID_CELLS: i32 = 2; // Total = ASTEROID_CELLS² * 4
        const ASTEROID_DISTANCE: f32 = 60.0;

        for ix in 0..ASTEROID_CELLS {
            for iy in 0..ASTEROID_CELLS {
                for (x, y) in [(ix, iy), (ix, -iy), (-ix, iy), (-ix, -iy)] {
                    let local_pos =
                        Vec2::new(x as f32 * ASTEROID_DISTANCE, y as f32 * ASTEROID_DISTANCE);

                    // TODO: randomize this a little
                    let velocity = asteroid_data.forward_velocity;

                    let despawn_after = calculate_milliseconds_until_asteroid_leaves_hexagon(
                        map_layout.hex_edge_vertices,
                        local_pos,
                        velocity,
                    );

                    spawn_helpers::spawn_asteroid(
                        &mut commands,
                        &sprites,
                        format!("Asteroid [{x},{y}]"),
                        &mut sector,
                        event.sector,
                        local_pos,
                        velocity,
                        (x * y) as f32 * 0.01,
                        now.add_milliseconds(despawn_after),
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

/// A big number to ensure the velocity vector is long enough to actually intersect with hexagon boundaries
const VELOCITY_MULTIPLIER: f32 = 100000000.0;

/// ## Panics
/// If the point is not within the hexagon or velocity is 0.
fn calculate_milliseconds_until_asteroid_leaves_hexagon(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_spawn_position: Vec2,
    velocity: Vec2,
) -> Milliseconds {
    debug_assert!(velocity.length_squared() > 0.0);
    let mut time = -1.0;

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = intersect_lines(
            local_spawn_position,
            local_spawn_position + velocity * VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance(local_spawn_position);
            if distance < 1.0 {
                // Too close, might happen when we are right on the edge
                continue;
            }

            time = distance / velocity.length();
            break;
        }
    }

    (time * 1000.0) as Milliseconds - 1 // Fade duration is ~1, so might as well subtract that for extra fancyness
}

fn calculate_asteroid_respawn_position(
    local_hexagon_edges: [[Vec2; 2]; 6],
    local_current_position: Vec2,
    velocity: Vec2,
) -> Vec2 {
    let mut best_distance = 0.0;
    let mut best_intersection = None;

    for edge in local_hexagon_edges.iter() {
        if let Some(intersection) = intersect_lines(
            local_current_position,
            local_current_position - velocity * VELOCITY_MULTIPLIER,
            edge[0],
            edge[1],
        ) {
            let distance = intersection.distance_squared(local_current_position);
            if distance > best_distance {
                best_distance = distance;
                best_intersection = Some(intersection);
            }
        }
    }

    best_intersection.expect("Asteroids should always intersect with their hexagon!")
}

#[derive(Resource, Default)]
pub struct FadingAsteroidsIn {
    pub asteroids: HashSet<AsteroidEntity>,
}
#[derive(Resource, Default)]
pub struct FadingAsteroidsOut {
    pub asteroids: HashSet<AsteroidEntity>,
}

/// Needs to run before [spawn_asteroids] in order to ensure no new asteroids are spawned which aren't yet synced.
/// Technically this doesn't need to run every frame, given the super slow speed of asteroids.
pub fn make_asteroids_disappear_when_they_leave_sector(
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut sector: Query<&mut Sector>,
    simulation_time: Res<SimulationTime>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        while let Some(next) = sector.asteroids.peek() {
            if now.has_not_passed(next.timestamp) {
                break;
            }

            let mut asteroid = sector.asteroids.pop().unwrap();
            asteroid
                .timestamp
                .add_milliseconds(constants::ASTEROID_RESPAWN_TIME_MILLISECONDS);
            fading_asteroids.asteroids.insert(asteroid.entity);
            sector.asteroid_respawns.push(asteroid);
        }
    }
}

pub fn respawn_asteroids(
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut sector: Query<&mut Sector>,
    mut asteroid_query: Query<(&mut Transform, &mut Visibility, &ConstantVelocity), With<Asteroid>>,
    simulation_time: Res<SimulationTime>,
    map_layout: Res<MapLayout>,
) {
    let now = simulation_time.now();

    for mut sector in sector.iter_mut() {
        while let Some(next) = sector.asteroid_respawns.peek() {
            if now.has_not_passed(next.timestamp) {
                break;
            }

            let mut asteroid = sector.asteroid_respawns.pop().unwrap();

            // TODO: figure out new asteroid position... just intersect the hexagon edge with velocity?
            // Or even easier, somehow clamp it to hexagon boundaries?
            // Either way, avoid randomness so we don't need to sync anything across the network

            let (mut transform, mut visibility, velocity) =
                asteroid_query.get_mut(asteroid.entity.into()).unwrap();

            let velocity = velocity.velocity.truncate();

            let local_respawn_position = calculate_asteroid_respawn_position(
                map_layout.hex_edge_vertices,
                transform.translation.truncate() - sector.world_pos,
                velocity,
            );

            let time = calculate_milliseconds_until_asteroid_leaves_hexagon(
                map_layout.hex_edge_vertices,
                local_respawn_position,
                velocity,
            );
            *visibility = Visibility::Inherited;
            transform.translation =
                (local_respawn_position + sector.world_pos).extend(constants::ASTEROID_LAYER);
            asteroid.timestamp.add_milliseconds(time);
            fading_asteroids.asteroids.insert(asteroid.entity);
            sector.asteroids.push(asteroid);
        }
    }
}

/// Fades asteroid alpha values to 0 before finally turning their visibility off.
pub fn fade_asteroids_out(
    time: Res<Time>,
    mut fading_asteroids: ResMut<FadingAsteroidsOut>,
    mut asteroid_query: Query<(&mut Sprite, &mut Visibility), With<Asteroid>>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let (mut sprite, mut visibility) = asteroid_query.get_mut(entity.into()).unwrap();

        let new_alpha = sprite.color.alpha() - time.delta_seconds();
        if new_alpha > 0.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            sprite.color.set_alpha(0.0);
            *visibility = Visibility::Hidden;
            removals.insert(*entity);
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}

/// Fades asteroids alpha values to 1
pub fn fade_asteroids_in(
    time: Res<Time>,
    mut fading_asteroids: ResMut<FadingAsteroidsIn>,
    mut asteroid_query: Query<&mut Sprite, With<Asteroid>>,
) {
    let mut removals = HashSet::new();

    for entity in &fading_asteroids.asteroids {
        let mut sprite = asteroid_query.get_mut(entity.into()).unwrap();

        let new_alpha = sprite.color.alpha() + time.delta_seconds();
        if new_alpha < 1.0 {
            sprite.color.set_alpha(new_alpha);
        } else {
            sprite.color.set_alpha(1.0);
            removals.insert(*entity);
        }
    }

    fading_asteroids.asteroids.retain(|x| !removals.contains(x));
}
