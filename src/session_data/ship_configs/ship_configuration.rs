use crate::game_data::{RecipeElement, ShipHullId, ShipHullManifest};
use crate::session_data::ShipConfigId;
use crate::simulation::prelude::Milliseconds;
use serde::Deserialize;

/// Defines the individual parts from which a ship is built.
///
/// Multiple ships can share the same configuration through their ID field.
#[derive(Deserialize)]
pub struct ShipConfiguration {
    pub id: ShipConfigId,
    pub name: String,
    pub parts: ShipConfigurationParts,
    pub computed_stats: ShipConfigurationComputedStats,
}

impl ShipConfiguration {
    pub fn from(
        id: ShipConfigId,
        name: String,
        parts: ShipConfigurationParts,
        ship_hulls: &ShipHullManifest,
    ) -> Self {
        let computed_stats = parts.compute_stats(ship_hulls);
        Self {
            id,
            name,
            parts,
            computed_stats,
        }
    }
}

#[derive(Deserialize)]
pub struct ShipConfigurationParts {
    pub hull: ShipHullId,
}

impl ShipConfigurationParts {
    pub fn compute_stats(&self, ship_hulls: &ShipHullManifest) -> ShipConfigurationComputedStats {
        let hull = ship_hulls.get_by_ref(&self.hull).unwrap();

        ShipConfigurationComputedStats {
            inventory_size: hull.inventory_size,
            build_time: hull.build_time,
            required_materials: hull.required_materials.clone(),
            asteroid_mining_amount: Some(10),
            gas_harvesting_amount: Some(10),
        }
    }
}

// TODO: Shouldn't be (de-)serialized, instead parsed from raw ship config data
#[derive(Deserialize)]
pub struct ShipConfigurationComputedStats {
    pub build_time: Milliseconds,
    pub required_materials: Vec<RecipeElement>,
    pub inventory_size: u32,
    pub asteroid_mining_amount: Option<u32>,
    pub gas_harvesting_amount: Option<u32>,
}
