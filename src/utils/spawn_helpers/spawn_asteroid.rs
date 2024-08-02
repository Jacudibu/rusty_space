use crate::components::{Asteroid, SectorAsteroidComponent, SelectableEntity};
use crate::persistence::{AsteroidIdMap, PersistentAsteroidId};
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::prelude::SimulationTimestamp;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{AsteroidEntity, AsteroidEntityWithTimestamp, SectorEntity};
use crate::{constants, SpriteHandles};
use bevy::color::Color;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Rot2, Sprite, SpriteBundle};

#[allow(clippy::too_many_arguments)]
pub fn spawn_asteroid(
    commands: &mut Commands,
    asteroid_id_map: &mut AsteroidIdMap,
    sprites: &SpriteHandles,
    name: String,
    global_pos: Vec2,
    asteroid_feature: &mut SectorAsteroidComponent,
    sector_entity: SectorEntity,
    velocity: Vec2,
    ore_current: u32,
    ore_max: u32,
    sprite_rotation: f32,
    angular_velocity: f32,
    despawn_at: SimulationTimestamp,
    fading_in: bool,
) -> AsteroidEntity {
    let asteroid_id = PersistentAsteroidId::next();
    let asteroid = Asteroid::new(asteroid_id, ore_current, ore_max, despawn_at);

    let simulation_transform = SimulationTransform::new(global_pos, Rot2::radians(sprite_rotation));

    let entity = commands
        .spawn((
            Name::new(name),
            SelectableEntity::Asteroid,
            ConstantVelocity::new(velocity, angular_velocity),
            SpriteBundle {
                texture: sprites.asteroid.clone(),
                transform: simulation_transform.as_transform(constants::z_layers::ASTEROID),
                sprite: Sprite {
                    color: Color::linear_rgba(1.0, 1.0, 1.0, if fading_in { 0.0 } else { 1.0 }),
                    ..Default::default()
                },
                ..Default::default()
            },
            simulation_transform,
            SimulationScale::default(),
            asteroid,
        ))
        .id();

    asteroid_id_map.insert(asteroid_id, AsteroidEntity::from(entity));
    asteroid_feature.add_asteroid(
        commands,
        sector_entity,
        AsteroidEntityWithTimestamp {
            entity: AsteroidEntity::from(entity),
            timestamp: despawn_at,
        },
    );

    AsteroidEntity::from(entity)
}
