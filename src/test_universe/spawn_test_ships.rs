use bevy::math::Vec2;
use bevy::prelude::{Commands, Query, Res};

use crate::components::Sector;
use crate::ship_ai::{AutoMineBehavior, AutoTradeBehavior};
use crate::test_universe::plugin::TestSectors;
use crate::utils::spawn_helpers;
use crate::{constants, SpriteHandles};

pub fn spawn_test_ships(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut sector_query: Query<&mut Sector>,
    debug_sectors: Res<TestSectors>,
) {
    for i in 0..constants::TRADE_SHIP_COUNT {
        spawn_helpers::spawn_ship(
            &mut commands,
            &sprites,
            format!("Trade Ship {i}"),
            &mut sector_query,
            debug_sectors.center,
            Vec2::ZERO,
            ((std::f32::consts::PI * 2.0) / constants::TRADE_SHIP_COUNT as f32) * i as f32,
            AutoTradeBehavior::default(),
        )
    }

    for i in 0..constants::MINING_SHIP_COUNT {
        spawn_helpers::spawn_ship(
            &mut commands,
            &sprites,
            format!("Mining Ship {i}"),
            &mut sector_query,
            debug_sectors.top_right,
            Vec2::ZERO,
            ((std::f32::consts::PI * 2.0) / constants::MINING_SHIP_COUNT as f32) * i as f32,
            AutoMineBehavior::default(),
        )
    }
}
