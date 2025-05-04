use crate::game_data::RecipeElement;
use crate::simulation_time::Milliseconds;
use crate::types::ship_size::ShipSize;
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use serde::Deserialize;
use std::path::PathBuf;

/// Raw data which will be parsed into [ShipHullData] on game start.
#[derive(Asset, TypePath, Deserialize)]
pub struct RawShipHullData {
    /// User facing name thingy
    pub name: String,

    /// Path of the sprite associated to this hull.
    pub sprite: PathBuf,

    /// The size class of this ship.
    pub ship_size: ShipSize,

    /// How many items fit into this ship by default.
    pub inventory_size: u32,

    /// Base values for the general maneuverability of this ship hull.
    pub maneuverability: ShipManeuverability,

    /// The amount of weapons which can be fitted onto this hull.
    pub weapon_slots: u8,

    /// Bill of materials required to build this, without modules.
    pub required_materials: Vec<RecipeElement>,

    /// How long this hull takes to build.
    pub build_time: Milliseconds,
}

/// Base values for the engine strength of a ship hull.
#[derive(Deserialize)]
pub struct ShipManeuverability {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}
