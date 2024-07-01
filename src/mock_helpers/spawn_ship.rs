use crate::components::{Engine, Inventory, SelectableEntity, Velocity};
use crate::sectors::{AllSectors, InSector, SectorData};
use crate::ship_ai::{AutoTradeBehavior, Idle};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, BuildChildren, Commands, Entity, Res, SpriteBundle, Transform};
use hexx::Hex;

pub fn spawn_ship(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    sector: &SectorData,
    position: Vec2,
    rotation: f32,
) {
    commands
        .spawn((
            Name::new(name),
            SelectableEntity::Ship,
            InSector::from(sector),
            AutoTradeBehavior,
            Idle::default(),
            Engine::default(),
            Velocity::default(),
            Inventory::new(100),
            SpriteBundle {
                texture: sprites.ship.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    translation: position.extend(constants::SHIP_LAYER),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(sector.entity);
}

pub fn spawn_mock_ships(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    sectors: Res<AllSectors>,
) {
    let start_sector = &sectors[&Hex::ZERO];
    for i in 0..constants::SHIP_COUNT {
        spawn_ship(
            &mut commands,
            &sprites,
            format!("Ship {i}"),
            start_sector,
            Vec2::ZERO,
            ((std::f32::consts::PI * 2.0) / constants::SHIP_COUNT as f32) * i as f32,
        )
    }
}
