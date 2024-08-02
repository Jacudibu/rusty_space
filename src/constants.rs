use crate::simulation::prelude::Milliseconds;
use std::ops::Range;

#[cfg(debug_assertions)]
pub const TRADE_SHIP_COUNT: u32 = 20;
#[cfg(not(debug_assertions))]
pub const TRADE_SHIP_COUNT: u32 = 200000;

pub const MINING_SHIP_COUNT: u32 = 20;
pub const HARVESTING_SHIP_COUNT: u32 = 20;
pub const ASTEROID_COUNT: usize = 400;

/// Lower values here mean ships will spread out more when buying / selling things, but also come to a halt much sooner,
/// as prices reach an equilibrium and resource consumption just isn't high enough yet.
#[cfg(debug_assertions)]
pub const MOCK_STATION_INVENTORY_SIZE: u32 = 5000;
#[cfg(not(debug_assertions))]
pub const MOCK_STATION_INVENTORY_SIZE: u32 = 39000000;

pub const SECONDS_TO_TRAVEL_THROUGH_GATE: f32 = 2.0;

pub const ASTEROID_ORE_RANGE: Range<u32> = 200..500;
pub const ASTEROID_VELOCITY_RANDOM_RANGE: Range<f32> = 0.8..1.2;
pub const ASTEROID_ROTATION_RANDOM_RANGE: Range<f32> = -0.001..0.001;

pub const TICKS_PER_SECOND: f64 = 10.0;

/// How big should our sectors be?
pub const SECTOR_SIZE: f32 = 500.0;

/// How much of [SECTOR_SIZE] is actually part of the sector. This is where the borders are drawn and stuff starts despawning.
pub const SECTOR_AREA_PERCENTAGE: f32 = 0.99;

pub const ASTEROID_RESPAWN_TIME: Milliseconds = 5000;

/// Basically a multiplier for orbit speeds
pub const GRAVITATIONAL_CONSTANT: f32 = 0.000000067;

pub const SIMULTANEOUS_STATION_INTERACTIONS: u32 = 4;
pub const SIMULTANEOUS_PLANET_INTERACTIONS: u32 = 8;
pub const DOCKING_DISTANCE_TO_STATION: f32 = 24.0;
pub const DOCKING_DISTANCE_TO_STATION_SQUARED: f32 =
    DOCKING_DISTANCE_TO_STATION * DOCKING_DISTANCE_TO_STATION;

pub mod z_layers {
    pub const SHIP: f32 = 10.0;
    pub const STATION: f32 = 5.0;
    pub const STATION_ICON: f32 = STATION + 1.0;
    pub const GATE: f32 = 5.0;
    pub const ASTEROID: f32 = 0.0;
    pub const PLANET_AND_STARS: f32 = 5.0;

    /// Sadly linestrip depth seems to be ignored by 2D Cameras. Right now this constant purely exists to avoid magic numbers.
    pub const GATE_CONNECTION: f32 = 0.0;
}
