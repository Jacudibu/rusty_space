use crate::components::{Asteroid, AsteroidState, Sector, SelectableEntity};
use crate::persistence::{AsteroidIdMap, PersistentAsteroidId};
use crate::simulation::physics::ConstantVelocity;
use crate::simulation::prelude::SimulationTimestamp;
use crate::simulation::ship_ai::AutoTradeBehavior;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{AsteroidEntity, AsteroidEntityWithTimestamp, SectorEntity};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Rot2, SpriteBundle};

#[allow(clippy::too_many_arguments)]
pub fn spawn_asteroid(
    commands: &mut Commands,
    asteroid_id_map: &mut AsteroidIdMap,
    sprites: &SpriteHandles,
    name: String,
    sector: &mut Sector,
    sector_entity: SectorEntity,
    local_position: Vec2,
    velocity: Vec2,
    ore_amount: u32,
    sprite_rotation: f32,
    despawn_at: SimulationTimestamp,
) {
    let asteroid_id = PersistentAsteroidId::next();
    let asteroid = Asteroid::new(
        asteroid_id,
        ore_amount,
        AsteroidState::Spawned { until: despawn_at },
    );

    let simulation_transform = SimulationTransform::new(
        sector.world_pos + local_position,
        Rot2::radians(sprite_rotation * std::f32::consts::PI * 1000.0),
        asteroid.scale_depending_on_current_ore_volume(),
    );

    let entity = commands
        .spawn((
            Name::new(name),
            SelectableEntity::Asteroid,
            AutoTradeBehavior::default(),
            ConstantVelocity::new(velocity, sprite_rotation),
            SpriteBundle {
                texture: sprites.asteroid.clone(),
                transform: simulation_transform.as_transform(constants::ASTEROID_LAYER),
                ..default()
            },
            simulation_transform,
            asteroid,
        ))
        .id();

    asteroid_id_map.insert(asteroid_id, AsteroidEntity::from(entity));
    sector.add_asteroid(
        commands,
        sector_entity,
        AsteroidEntityWithTimestamp {
            entity: AsteroidEntity::from(entity),
            timestamp: despawn_at,
        },
    );
}
