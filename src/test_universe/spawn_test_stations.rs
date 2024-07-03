use bevy::math::Vec2;
use bevy::prelude::{Commands, Query, Res};

use crate::components::Sector;
use crate::game_data::{
    GameData, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B, DEBUG_ITEM_ID_C, PRODUCTION_MODULE_A_ID,
    PRODUCTION_MODULE_B_ID, PRODUCTION_MODULE_C_ID, RECIPE_A_ID, RECIPE_B_ID, RECIPE_C_ID,
};
use crate::spawn_helpers::{MockStationProductionArgElement, MockStationProductionArgs};
use crate::test_universe::plugin::TestSectors;
use crate::{spawn_helpers, SpriteHandles};

pub fn spawn_test_stations(
    mut commands: Commands,
    mut sector_query: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    game_data: Res<GameData>,
    debug_sectors: Res<TestSectors>,
) {
    spawn_helpers::spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station A",
        Vec2::new(-200.0, -200.0),
        debug_sectors.bottom_left,
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_B_ID, RECIPE_B_ID, 5),
        ])),
        None,
    );
    spawn_helpers::spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station B",
        Vec2::new(200.0, -200.0),
        debug_sectors.center,
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_C_ID, RECIPE_C_ID, 3),
        ])),
        None,
    );
    spawn_helpers::spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station C",
        Vec2::new(0.0, 200.0),
        debug_sectors.center,
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_A_ID, RECIPE_A_ID, 1),
        ])),
        None,
    );
    spawn_helpers::spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Shipyard",
        Vec2::new(0.0, 0.0),
        debug_sectors.center,
        vec![
            &game_data.items[&DEBUG_ITEM_ID_A],
            &game_data.items[&DEBUG_ITEM_ID_B],
            &game_data.items[&DEBUG_ITEM_ID_C],
        ],
        vec![],
        None,
        Some(true),
    );
}
