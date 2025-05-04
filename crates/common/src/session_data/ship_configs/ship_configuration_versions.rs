use crate::session_data::ShipConfiguration;
use crate::session_data::ship_configs::version;
use crate::session_data::ship_configs::version::Version;
use bevy::platform::collections::HashMap;
use serde::Deserialize;
use std::hash::{Hash, Hasher};

/// Keeps track of all versions which have been created over time. Older versions which are no longer in use can be deleted without anything breaking.
#[derive(Deserialize)]
pub struct ShipConfigurationVersions {
    pub versions: HashMap<Version, ShipConfiguration>,

    // TODO: Might wanna split latest into "active" and "max" in the future - if versions can be deleted, latest might be deleted as well.
    //       Or a different version might be selected as the "active" one, causing max != latest.
    latest: Version,
}

impl Hash for ShipConfigurationVersions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.latest.hash(state);
    }
}

impl ShipConfigurationVersions {
    pub fn latest(&self) -> &ShipConfiguration {
        &self.versions[&self.latest]
    }

    pub fn get_version(&self, version: &Version) -> Option<&ShipConfiguration> {
        self.versions.get(version)
    }

    /// Crates a new [ShipConfigurationVersions] collection from a given [ShipConfiguration]
    pub fn new(first_version: ShipConfiguration) -> Self {
        Self {
            versions: HashMap::from([(version::INITIAL_VERSION, first_version)]),
            latest: version::INITIAL_VERSION,
        }
    }

    /// Adds a new [ShipConfiguration] to the collection and sets it as the latest version.
    pub fn add_as_latest(&mut self, value: ShipConfiguration) {
        let version = self.next_version();
        self.versions.insert(version, value);
        self.latest = version;
    }

    #[inline]
    #[must_use]
    pub fn next_version(&self) -> Version {
        self.latest.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_data::SHIP_HULL_TRANSPORT_ID;
    use crate::session_data::ShipConfigId;
    use crate::session_data::ship_configs::ship_configuration::{
        ShipConfigurationComputedStats, ShipConfigurationParts,
    };
    use crate::session_data::ship_configs::{EngineStats, EngineTuning};

    fn mock_parts() -> ShipConfigurationParts {
        ShipConfigurationParts {
            hull: SHIP_HULL_TRANSPORT_ID,
            weapons: vec![],
        }
    }

    fn mock_stats() -> ShipConfigurationComputedStats {
        ShipConfigurationComputedStats {
            build_time: 5,
            required_materials: Vec::new(),
            inventory_size: 10,
            engine: EngineStats {
                max_speed: 100.0,
                acceleration: 10.0,
                deceleration: 30.0,
                max_angular_speed: 1.0,
                angular_acceleration: 1.0,
            },
            build_power: None,
            asteroid_mining_amount: None,
            gas_harvesting_amount: None,
        }
    }

    #[test]
    fn add_as_latest() {
        let name = "test";

        let mut versions = ShipConfigurationVersions::new(ShipConfiguration {
            id: ShipConfigId::from_name(name),
            name: "Test".into(),
            parts: mock_parts(),
            engine_tuning: EngineTuning::default(),
            computed_stats: mock_stats(),
            sprite: Default::default(),
            sprite_selected: Default::default(),
        });

        let next_version = versions.next_version();
        versions.add_as_latest(ShipConfiguration {
            id: ShipConfigId::from_name_and_version(name, next_version),
            name: "Test".into(),
            parts: mock_parts(),
            engine_tuning: EngineTuning::default(),
            computed_stats: mock_stats(),
            sprite: Default::default(),
            sprite_selected: Default::default(),
        });

        assert_eq!(next_version, versions.latest().id.version);
    }
}
