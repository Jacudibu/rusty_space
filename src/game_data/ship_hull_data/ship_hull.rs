use crate::game_data::RecipeElement;
use crate::game_data::ship_hull_data::ShipHullId;
use crate::game_data::ship_hull_data::raw_ship_hull::ShipManeuverability;
use crate::simulation::prelude::Milliseconds;
use crate::utils::ShipSize;
use bevy::prelude::{Handle, Image};

/// Defines the base of a ship.
pub struct ShipHullData {
    /// Unique ID to differentiate between recipes
    pub id: ShipHullId,

    /// User Facing name thingy
    pub name: String,

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

    /// The sprite that's associated with this ship hull.
    pub sprite: Handle<Image>,
}
