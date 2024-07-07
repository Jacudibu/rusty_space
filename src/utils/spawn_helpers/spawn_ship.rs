use crate::components::{Engine, Inventory, Sector, SelectableEntity, Ship};
use crate::physics::ShipVelocity;
use crate::ship_ai::TaskQueue;
use crate::utils::{SectorEntity, ShipEntity};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, Component, Query, SpriteBundle, Transform};

pub fn spawn_ship<TBehavior: Component>(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    sector_query: &mut Query<&mut Sector>,
    sector: SectorEntity,
    position: Vec2,
    rotation: f32,
    behavior: TBehavior,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();

    let entity = commands
        .spawn((
            Name::new(name),
            Ship,
            SelectableEntity::Ship,
            behavior,
            Engine::default(),
            ShipVelocity::default(),
            Inventory::new(100),
            TaskQueue::new(),
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

    sector_data.add_ship(commands, sector, ShipEntity::from(entity));
}
