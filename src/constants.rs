use crate::utils::Milliseconds;
use std::ops::Range;

pub const TRADE_SHIP_COUNT: u32 = 10;
pub const MINING_SHIP_COUNT: u32 = 50;
pub const ASTEROID_COUNT: usize = 400;

pub const MOCK_STATION_INVENTORY_SIZE: u32 = 500000;
pub const SECONDS_TO_TRAVEL_THROUGH_GATE: f32 = 2.0;

pub const ASTEROID_ORE_RANGE: Range<u32> = 100..500;

pub const SHIP_LAYER: f32 = 10.0;
pub const STATION_LAYER: f32 = 5.0;
pub const STATION_ICON_LAYER: f32 = 6.0;
pub const GATE_LAYER: f32 = 5.0;
pub const ASTEROID_LAYER: f32 = 0.0;

/// Sadly linestrip depth seems to be ignored by 2D Cameras. Right now this constant purely exists to avoid magic numbers.
pub const GATE_CONNECTION_LAYER: f32 = 0.0;

/// How big should our sectors be?
pub const SECTOR_SIZE: f32 = 500.0;

/// How much of [SECTOR_SIZE] is actually part of the sector. This is where the borders are drawn and stuff starts despawning.
pub const SECTOR_AREA_PERCENTAGE: f32 = 0.99;

pub const ASTEROID_RESPAWN_TIME_MILLISECONDS: Milliseconds = 5000;
