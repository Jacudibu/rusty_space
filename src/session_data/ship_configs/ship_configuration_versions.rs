use crate::session_data::ship_configs::version;
use crate::session_data::ship_configs::version::Version;
use crate::session_data::ShipConfiguration;
use bevy::utils::HashMap;
use serde::Deserialize;

/// Keeps track of all versions which have been created over time. Older versions which are no longer in use can be deleted without anything breaking.
#[derive(Deserialize)]
pub struct ShipConfigurationVersions {
    versions: HashMap<Version, ShipConfiguration>,

    // TODO: Might wanna split latest into "active" and "max" in the future - if versions can be deleted, latest might be deleted as well.
    //       Or a different version might be selected as the "active" one, causing max != latest.
    latest: Version,
}

impl ShipConfigurationVersions {
    pub fn latest(&self) -> &ShipConfiguration {
        &self.versions[&self.latest]
    }

    pub fn get_version(&self, version: &Version) -> Option<&ShipConfiguration> {
        self.versions.get(version)
    }

    pub fn new(first_version: ShipConfiguration) -> Self {
        Self {
            versions: HashMap::from([(version::INITIAL_VERSION, first_version)]),
            latest: version::INITIAL_VERSION,
        }
    }

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
    use crate::game_data::MOCK_SHIP_HULL_A_ID;
    use crate::session_data::ship_configs::ship_configuration::{
        ShipConfigurationComputedStats, ShipConfigurationParts,
    };
    use crate::session_data::ShipConfigId;

    fn mock_parts() -> ShipConfigurationParts {
        ShipConfigurationParts {
            hull: MOCK_SHIP_HULL_A_ID,
        }
    }

    fn mock_stats() -> ShipConfigurationComputedStats {
        ShipConfigurationComputedStats {
            build_time: 5,
            required_materials: Vec::new(),
            inventory_size: 10,
        }
    }

    #[test]
    fn add_as_latest() {
        let name = "test";

        let mut versions = ShipConfigurationVersions::new(ShipConfiguration {
            id: ShipConfigId::from_name(name),
            name: "Test".into(),
            parts: mock_parts(),
            computed_stats: mock_stats(),
        });

        let next_version = versions.next_version();
        versions.add_as_latest(ShipConfiguration {
            id: ShipConfigId::from_name_and_version(name, next_version),
            name: "Test".into(),
            parts: mock_parts(),
            computed_stats: mock_stats(),
        });

        assert_eq!(next_version, versions.latest().id.version);
    }
}
