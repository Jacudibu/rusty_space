use serde::{Deserialize, Serialize};

pub const INITIAL_VERSION: Version = Version { version: 1 };

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Version {
    version: u32,
}

impl Version {
    #[must_use]
    pub fn next(&self) -> Self {
        Self {
            version: self.version + 1,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        INITIAL_VERSION
    }
}
