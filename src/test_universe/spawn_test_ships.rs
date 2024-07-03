use bevy::math::Vec2;
use bevy::prelude::{Commands, Query, Res};

use crate::components::Sector;
use crate::test_universe::plugin::TestSectors;
use crate::{constants, spawn_helpers, SpriteHandles};

pub fn spawn_test_ships(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_query: Query<&mut Sector>,
    debug_sectors: Res<TestSectors>,
) {
    for i in 0..constants::SHIP_COUNT {
        spawn_helpers::spawn_ship(
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
