use crate::components::{Engine, Inventory, SelectableEntity, Velocity};
use crate::sectors::{Sector, SectorEntity};
use crate::ship_ai::{AutoTradeBehavior, Idle};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, Query, SpriteBundle, Transform};

pub fn spawn_ship(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    sector_query: &mut Query<&mut Sector>,
    sector: SectorEntity,
    position: Vec2,
    rotation: f32,
) {
    let mut sector_data = sector_query.get_mut(sector.get()).unwrap();

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

    sector_data.add_ship(commands, sector, entity);
}