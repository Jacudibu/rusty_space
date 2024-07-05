use crate::components::{Asteroid, Sector, SectorAsteroidData, SelectableEntity};
use crate::physics::{ConstantVelocity, ShipVelocity};
use crate::ship_ai::AutoTradeBehavior;
use crate::utils::{
    AsteroidEntity, AsteroidEntityWithTimestamp, SectorEntity, SimulationTimestamp,
};
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
    local_position: Vec2,
    velocity: Vec2,
    sprite_rotation: f32,
    despawn_at: SimulationTimestamp,
) {
    let entity = commands
        .spawn((
            Name::new(name),
            Asteroid { ore: 100 },
            SelectableEntity::Asteroid,
            AutoTradeBehavior::default(),
            ConstantVelocity::new(velocity, sprite_rotation),
            SpriteBundle {
                texture: sprites.asteroid.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(sprite_rotation),
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
        AsteroidEntityWithTimestamp {
            entity: AsteroidEntity::from(entity),
            timestamp: despawn_at,
        },
    );
}
