use crate::components::{Engine, Inventory, Sector, SelectableEntity, Ship};
use crate::persistence::PersistentShipId;
use crate::physics::ShipVelocity;
use crate::ship_ai::{BehaviorBuilder, TaskQueue};
use crate::utils::{SectorEntity, ShipEntity};
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
    behavior: &BehaviorBuilder,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();

    let ship_id = PersistentShipId::next();
    let entity = commands
        .spawn((
            Name::new(name),
            Ship::new(ship_id),
            SelectableEntity::Ship,
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

    // TODO: There must be *some* way to build that component earlier and insert it at spawn time...?
    behavior.build_and_add_default_component(commands.entity(entity));

    sector_data.add_ship(commands, sector, ShipEntity::from(entity));
}
