use crate::components::{
    AutoTradeData, Engine, Inventory, SelectableEntity, ShipBehavior, Velocity,
};
use crate::ship_ai::Idle;
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform};

pub fn spawn_ship(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: String,
    position: Vec2,
    rotation: f32,
) {
    commands.spawn((
        Name::new(name),
        SelectableEntity::Ship,
        ShipBehavior::AutoTrade(AutoTradeData {}),
        Idle,
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
    ));
}

pub fn spawn_mock_ships(mut commands: Commands, sprites: Res<SpriteHandles>) {
    for i in 0..constants::SHIP_COUNT {
        spawn_ship(
            &mut commands,
            &sprites,
            format!("Ship {i}"),
            Vec2::ZERO,
            ((std::f32::consts::PI * 2.0) / constants::SHIP_COUNT as f32) * i as f32,
        )
    }
}
