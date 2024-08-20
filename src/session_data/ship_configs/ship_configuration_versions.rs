use crate::session_data::ship_configs::version;
use crate::session_data::ship_configs::version::Version;
use crate::session_data::ShipConfiguration;
use bevy::utils::HashMap;
use serde::Deserialize;

/// Keeps track of all versions which have been created over time. Older versions which are no longer in use can be deleted without anything breaking.
#[derive(Deserialize)]
pub struct ShipConfigurationVersions {
    versions: HashMap<Version, ShipConfiguration>,
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
        Version::from_predecessor(self.latest)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::session_data::ShipConfigId;

    #[test]
    fn add_as_latest() {
        let name = "test";
        let mut versions = ShipConfigurationVersions::new(ShipConfiguration {
            id: ShipConfigId::from_name(name),
            name: "Test".into(),
            duration: 5,
            materials: Vec::default(),
        });

        let next_version = versions.next_version();
        versions.add_as_latest(ShipConfiguration {
            id: ShipConfigId::from_name_and_version(name, next_version),
            name: "Test".into(),
            duration: 5,
            materials: Vec::default(),
        });

        assert_eq!(next_version, versions.latest().id.version);
    }
}
