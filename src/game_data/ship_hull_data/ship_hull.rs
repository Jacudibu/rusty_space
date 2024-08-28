use crate::game_data::generic_manifest_without_raw_data::DataCanBeUsedAsRawData;
use crate::game_data::ship_hull_data::ShipHullId;
use crate::game_data::RecipeElement;
use crate::simulation::prelude::Milliseconds;
use crate::utils::ShipSize;
use bevy::prelude::TypePath;
use serde::Deserialize;

/// Defines the base of a ship.
#[derive(TypePath, Deserialize)]
pub struct ShipHullData {
    /// Unique ID to differentiate between recipes
    pub id: ShipHullId,

    /// User Facing name thingy
    pub name: String,

    /// The size class of this ship.
    pub ship_size: ShipSize,

    /// How many items fit into this ship by default.
    pub inventory_size: u32,

    /// Base values for the general maneuverability of a ship hull.
    pub maneuverability: ShipManeuverability,

    /// The amount of weapons which can be fitted onto this hull.
    pub weapon_slots: u8,

    /// Bill of materials required to build this, without modules.
    pub required_materials: Vec<RecipeElement>,

    /// How long this hull takes to build.
    pub build_time: Milliseconds,
}

impl DataCanBeUsedAsRawData for ShipHullData {}

/// Base values for the engine strength of a ship hull.
#[derive(Deserialize)]
pub struct ShipManeuverability {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}
