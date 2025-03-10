use crate::game_data::{
    RecipeElement, ShipHullData, ShipHullId, ShipHullManifest, ShipWeaponId, ShipWeaponManifest,
};
use crate::image_generator;
use crate::session_data::ShipConfigId;
use crate::simulation::prelude::Milliseconds;
use bevy::prelude::{Assets, Handle, Image};
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
    pub engine_tuning: EngineTuning,

    // TODO: These sprites should be customizable per config by the user to some degree
    #[serde(skip)]
    pub sprite: Handle<Image>,
    #[serde(skip)]
    pub sprite_selected: Handle<Image>,
}

impl ShipConfiguration {
    pub fn from(
        id: ShipConfigId,
        name: String,
        parts: ShipConfigurationParts,
        ship_hulls: &ShipHullManifest,
        ship_weapons: &ShipWeaponManifest,
        image_assets: &mut Assets<Image>,
    ) -> Self {
        let engine_tuning = EngineTuning::default();
        let computed_stats = parts.compute_stats(&engine_tuning, ship_hulls, ship_weapons);

        let sprite = ship_hulls.get_by_ref(&parts.hull).unwrap().sprite.clone();
        let sprite_selected = image_generator::generate_image_with_highlighted_corners_from_handle(
            &sprite,
            image_assets,
        );

        Self {
            id,
            sprite,
            sprite_selected,
            name,
            parts,
            engine_tuning,
            computed_stats,
        }
    }
}

/// The individual parts making up a ShipConfiguration.
#[derive(Deserialize)]
pub struct ShipConfigurationParts {
    pub hull: ShipHullId,
    pub weapons: Vec<ShipWeaponId>,
}

impl ShipConfigurationParts {
    pub fn compute_stats(
        &self,
        tuning: &EngineTuning,
        ship_hulls: &ShipHullManifest,
        ship_weapons: &ShipWeaponManifest,
    ) -> ShipConfigurationComputedStats {
        let hull = ship_hulls.get_by_ref(&self.hull).unwrap();
        let weapons: Vec<_> = self
            .weapons
            .iter()
            .filter_map(|x| ship_weapons.get_by_ref(x))
            .collect();

        ShipConfigurationComputedStats {
            inventory_size: hull.inventory_size,
            build_time: hull.build_time,
            required_materials: hull.required_materials.clone(), // TODO: sum up all materials
            build_power: Self::sum_strength(&weapons, |x| x.build_power),
            asteroid_mining_amount: Self::sum_strength(&weapons, |x| x.asteroid_mining_strength),
            gas_harvesting_amount: Self::sum_strength(&weapons, |x| x.gas_harvesting_strength),
            engine: EngineStats::compute_from(hull, tuning),
        }
    }

    fn sum_strength<X, T>(items: &[&X], value_getter: T) -> Option<u32>
    where
        T: Fn(&&X) -> Option<u32>,
    {
        let result = items.iter().filter_map(value_getter).sum();
        if result > 0 { Some(result) } else { None }
    }
}

/// The accumulated stats based on the given ConfigurationParts. Created by calling [`ShipConfiguration::compute_stats`].
// TODO: Shouldn't be (de-)serialized, instead parsed from raw ship config data
#[derive(Deserialize)]
pub struct ShipConfigurationComputedStats {
    pub build_time: Milliseconds,
    pub required_materials: Vec<RecipeElement>,
    pub inventory_size: u32,
    pub engine: EngineStats,
    pub build_power: Option<u32>,
    pub asteroid_mining_amount: Option<u32>,
    pub gas_harvesting_amount: Option<u32>,
}

// TODO: Shouldn't be (de-)serialized, instead parsed from raw ship config data
#[derive(Deserialize)]
pub struct EngineStats {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,

    pub max_angular_speed: f32,
    pub angular_acceleration: f32,
}

impl EngineStats {
    pub fn compute_from(hull: &ShipHullData, tuning: &EngineTuning) -> Self {
        Self {
            max_speed: hull.maneuverability.max_speed
                * Self::tuning_value_to_multiplier(tuning.max_speed),
            acceleration: hull.maneuverability.acceleration
                * Self::tuning_value_to_multiplier(tuning.acceleration),
            deceleration: hull.maneuverability.deceleration
                * Self::tuning_value_to_multiplier(tuning.acceleration),
            max_angular_speed: hull.maneuverability.max_angular_speed
                * Self::tuning_value_to_multiplier(tuning.turning),
            angular_acceleration: hull.maneuverability.angular_acceleration
                * Self::tuning_value_to_multiplier(tuning.turning),
        }
    }

    /// `value` should be in [0, 6].
    ///
    /// ## Returns
    /// `value` scaled to [0.9, 1.1].
    fn tuning_value_to_multiplier(value: u8) -> f32 {
        0.9 + (value as f32 / 6.0) * 0.2
    }
}

#[derive(Deserialize)]
pub struct EngineTuning {
    pub acceleration: u8,
    pub max_speed: u8,
    pub turning: u8,
}

impl Default for EngineTuning {
    fn default() -> Self {
        Self {
            turning: 6,
            max_speed: 6,
            acceleration: 6,
        }
    }
}

mod test {
    #[allow(unused)] // No clue why it complains about this...?
    use super::EngineStats;

    #[test]
    fn tuning_value_to_multiplier() {
        assert_eq!(0.9, EngineStats::tuning_value_to_multiplier(0));
        assert_eq!(1.0, EngineStats::tuning_value_to_multiplier(3));
        assert_eq!(1.1, EngineStats::tuning_value_to_multiplier(6));
    }
}
