use crate::session_data::ship_configs::version::{INITIAL_VERSION, Version};
use leafwing_manifest::identifier::Id;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
pub struct VersionedId<T> {
    pub id: Id<T>,
    pub version: Version,
}

impl<T> VersionedId<T> {
    #[must_use]
    pub const fn from_name(name: &str) -> Self {
        Self {
            id: Id::from_name(name),
            version: INITIAL_VERSION,
        }
    }

    #[must_use]
    pub const fn from_name_and_version(name: &str, version: Version) -> Self {
        Self {
            id: Id::from_name(name),
            version,
        }
    }
}

impl<T> Hash for VersionedId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.version.hash(state);
    }
}

impl<T> PartialEq for VersionedId<T> {
    fn eq(&self, other: &Self) -> bool {
        other.id.eq(&self.id) && other.version.eq(&self.version)
    }
}

impl<T> Eq for VersionedId<T> {}

impl<T> Debug for VersionedId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VersionedId")
            .field("id", &self.id)
            .field("version", &self.version)
            .finish()
    }
}

impl<T> Clone for VersionedId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for VersionedId<T> {}

impl<T> Serialize for VersionedId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("VersionedId", 2)?;
        serialize_struct.serialize_field("id", &self.id)?;
        serialize_struct.serialize_field("version", &self.version)?;
        serialize_struct.end()
    }
}
