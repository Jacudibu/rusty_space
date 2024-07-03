use crate::components::{Engine, Inventory, SelectableEntity, Velocity};
use crate::sectors::{DebugSectors, Sector};
use crate::ship_ai::{AutoTradeBehavior, Idle};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, Entity, Query, Res, SpriteBundle, Transform};

pub fn spawn_ship(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    sector_query: &mut Query<&mut Sector>,
    sector_entity: Entity,
    position: Vec2,
    rotation: f32,
) {
    let mut sector_data = sector_query.get_mut(sector_entity).unwrap();

    let entity = commands
        .spawn((
            Name::new(name),
            SelectableEntity::Ship,
            AutoTradeBehavior,
            Idle::default(),
            Engine::default(),
            Velocity::default(),
            Inventory::new(100),
            SpriteBundle {
                texture: sprites.ship.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    translation: (sector_data.world_pos + position).extend(constants::SHIP_LAYER),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    sector_data.add_ship(commands, sector_entity, entity);
}

pub fn spawn_mock_ships(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_query: Query<&mut Sector>,
    debug_sectors: Res<DebugSectors>,
) {
    for i in 0..constants::SHIP_COUNT {
        spawn_ship(
            &mut commands,
            &sprites,
            format!("Ship {i}"),
            &mut sector_query,
            debug_sectors.center,
            Vec2::ZERO,
            ((std::f32::consts::PI * 2.0) / constants::SHIP_COUNT as f32) * i as f32,
        )
    }
}
