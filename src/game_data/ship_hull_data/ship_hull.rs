use crate::game_data::ship_hull_data::ShipHullId;
use crate::game_data::RecipeElement;
use crate::simulation::prelude::Milliseconds;
use crate::utils::ShipSize;
use serde::Deserialize;

/// Defines the base of a ship.
#[derive(Deserialize)]
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
