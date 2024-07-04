pub const SHIP_COUNT: u32 = 200000;
pub const ASTEROID_COUNT: u32 = 10;

pub const MOCK_INVENTORY_SIZE: u32 = 500000000;
pub const SECONDS_TO_TRAVEL_THROUGH_GATE: f32 = 2.0;

pub const SHIP_LAYER: f32 = 10.0;
pub const STATION_LAYER: f32 = 5.0;
pub const GATE_LAYER: f32 = 5.0;

/// Sadly linestrip depth seems to be ignored by 2D Cameras. Right now this constant purely exists to avoid magic numbers.
pub const GATE_CONNECTION_LAYER: f32 = 0.0;

pub const SECTOR_SIZE: f32 = 500.0;
