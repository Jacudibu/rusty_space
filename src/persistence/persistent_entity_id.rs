use crate::components::{Asteroid, ConstructionSite, Gate, Planet, Ship, Station};
use bevy::prelude::Component;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};

/// A unique ID that's the same between session and across different clients in multiplayer sessions.
/// Should be used for persistence and networking.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum PersistentEntityId {
    Asteroid(PersistentAsteroidId),
    Gate(PersistentGateId),
    Planet(PersistentPlanetId),
    Ship(PersistentShipId),
    Station(PersistentStationId),
    ConstructionSite(PersistentConstructionSiteId),
    Sector(Hex),
}

#[derive(Serialize, Deserialize)]
pub struct TypedPersistentEntityId<T: Component>(u32, PhantomData<T>);

pub trait ComponentWithPersistentId<T: Component> {
    fn id(&self) -> TypedPersistentEntityId<T>;
}

macro_rules! impl_typed_persistent_entity_id {
    ($entity_type:ty, $name:ident) => {
        static $name: AtomicU32 = AtomicU32::new(0);

        impl TypedPersistentEntityId<$entity_type> {
            pub fn next() -> Self {
                Self($name.fetch_add(1, Ordering::Relaxed), PhantomData)
            }
        }
    };
}

macro_rules! impl_traits {
    ($entity_type:ty, $variant:ident) => {
        impl From<$entity_type> for PersistentEntityId {
            fn from(value: $entity_type) -> Self {
                Self::$variant(value)
            }
        }

        impl From<&$entity_type> for PersistentEntityId {
            fn from(value: &$entity_type) -> Self {
                Self::$variant(*value)
            }
        }
    };
}

/// A [PersistentEntityId] for [Asteroid]s.
pub type PersistentAsteroidId = TypedPersistentEntityId<Asteroid>;
impl_traits!(PersistentAsteroidId, Asteroid);
impl_typed_persistent_entity_id!(Asteroid, NEXT_ASTEROID_ID);

/// A [PersistentEntityId] for [Gate]s.
pub type PersistentGateId = TypedPersistentEntityId<Gate>;
impl_traits!(PersistentGateId, Gate);
impl_typed_persistent_entity_id!(Gate, NEXT_GATE_ID);

/// A [PersistentEntityId] for [Planet]s.
pub type PersistentPlanetId = TypedPersistentEntityId<Planet>;
impl_traits!(PersistentPlanetId, Planet);
impl_typed_persistent_entity_id!(Planet, NEXT_PLANET_ID);

/// A [PersistentEntityId] for [Ship]s.
pub type PersistentShipId = TypedPersistentEntityId<Ship>;
impl_traits!(PersistentShipId, Ship);
impl_typed_persistent_entity_id!(Ship, NEXT_SHIP_ID);

/// A [PersistentEntityId] for [Station]s.
pub type PersistentStationId = TypedPersistentEntityId<Station>;
impl_traits!(PersistentStationId, Station);
impl_typed_persistent_entity_id!(Station, NEXT_STATION_ID);

/// A [PersistentEntityId] for [ConstructionSite]s.
pub type PersistentConstructionSiteId = TypedPersistentEntityId<ConstructionSite>;
impl_traits!(PersistentConstructionSiteId, ConstructionSite);
impl_typed_persistent_entity_id!(ConstructionSite, NEXT_CONSTRUCTION_SITE_ID);

impl_traits!(Hex, Sector);

impl<T: Component> Copy for TypedPersistentEntityId<T> {}
impl<T: Component> Clone for TypedPersistentEntityId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Component> Eq for TypedPersistentEntityId<T> {}
impl<T: Component> PartialEq<Self> for TypedPersistentEntityId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Component> Ord for TypedPersistentEntityId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T: Component> PartialOrd<Self> for TypedPersistentEntityId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Component> Hash for TypedPersistentEntityId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Component> Display for TypedPersistentEntityId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: Component> Debug for TypedPersistentEntityId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", std::any::type_name::<T>(), self.0)
    }
}
