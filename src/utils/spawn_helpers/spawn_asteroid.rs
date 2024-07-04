use crate::components::{Asteroid, Sector, SectorAsteroidData, SelectableEntity, Velocity};
use crate::ship_ai::AutoTradeBehavior;
use crate::utils::{AsteroidEntity, AsteroidEntityWithLifetime, SectorEntity, SimulationTimestamp};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, SpriteBundle, Transform};

pub fn spawn_asteroid(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    sector: &mut Sector,
    sector_entity: SectorEntity,
    asteroid_data: &SectorAsteroidData,
    local_position: Vec2,
    rotation: f32,
    despawn_at: SimulationTimestamp,
) {
    let entity = commands
        .spawn((
            Name::new(name),
            Asteroid { ore: 100 },
            SelectableEntity::Asteroid,
            AutoTradeBehavior::default(),
            Velocity {
                forward: asteroid_data.forward_velocity,
                angular: 0.0,
            },
            SpriteBundle {
                texture: sprites.asteroid.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    translation: (sector.world_pos + local_position).extend(constants::SHIP_LAYER),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    sector.add_asteroid(
        commands,
        sector_entity,
        AsteroidEntityWithLifetime {
            entity: AsteroidEntity::from(entity),
            despawn_at,
        },
    );
}
