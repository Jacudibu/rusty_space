use crate::components::{Asteroid, Gate, Ship, Station};
use bevy::prelude::Component;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};

/// A unique ID that's the same between session and across different clients in multiplayer sessions.
/// (Sectors are just represented as Hex coordinates)
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum PersistentEntityId {
    Asteroid(PersistentAsteroidId),
    Gate(PersistentGateId),
    Ship(PersistentShipId),
    Station(PersistentStationId),
    Sector(Hex),
}

impl From<PersistentAsteroidId> for PersistentEntityId {
    fn from(value: PersistentAsteroidId) -> Self {
        Self::Asteroid(value)
    }
}

impl From<&PersistentAsteroidId> for PersistentEntityId {
    fn from(value: &PersistentAsteroidId) -> Self {
        Self::Asteroid(*value)
    }
}

impl From<PersistentGateId> for PersistentEntityId {
    fn from(value: PersistentGateId) -> Self {
        Self::Gate(value)
    }
}
impl From<&PersistentGateId> for PersistentEntityId {
    fn from(value: &PersistentGateId) -> Self {
        Self::Gate(*value)
    }
}

impl From<PersistentShipId> for PersistentEntityId {
    fn from(value: PersistentShipId) -> Self {
        Self::Ship(value)
    }
}
impl From<&PersistentShipId> for PersistentEntityId {
    fn from(value: &PersistentShipId) -> Self {
        Self::Ship(*value)
    }
}

impl From<PersistentStationId> for PersistentEntityId {
    fn from(value: PersistentStationId) -> Self {
        Self::Station(value)
    }
}

impl From<&PersistentStationId> for PersistentEntityId {
    fn from(value: &PersistentStationId) -> Self {
        Self::Station(*value)
    }
}

impl From<Hex> for PersistentEntityId {
    fn from(value: Hex) -> Self {
        Self::Sector(value)
    }
}

pub type PersistentAsteroidId = TypedPersistentEntityId<Asteroid>;
pub type PersistentGateId = TypedPersistentEntityId<Gate>;
pub type PersistentShipId = TypedPersistentEntityId<Ship>;
pub type PersistentStationId = TypedPersistentEntityId<Station>;

#[derive(Serialize, Deserialize)]
pub struct TypedPersistentEntityId<T: Component>(u32, PhantomData<T>);

pub trait ComponentWithPersistentId<T: Component> {
    fn id(&self) -> TypedPersistentEntityId<T>;
}

static NEXT_ASTEROID_ID: AtomicU32 = AtomicU32::new(0);
impl TypedPersistentEntityId<Asteroid> {
    pub fn next() -> Self {
        Self(
            NEXT_ASTEROID_ID.fetch_add(1, Ordering::Relaxed),
            PhantomData,
        )
    }
}

static NEXT_GATE_ID: AtomicU32 = AtomicU32::new(0);
impl TypedPersistentEntityId<Gate> {
    pub fn next() -> Self {
        Self(NEXT_GATE_ID.fetch_add(1, Ordering::Relaxed), PhantomData)
    }
}

static NEXT_SHIP_ID: AtomicU32 = AtomicU32::new(0);
impl TypedPersistentEntityId<Ship> {
    pub fn next() -> Self {
        Self(NEXT_SHIP_ID.fetch_add(1, Ordering::Relaxed), PhantomData)
    }
}

static NEXT_STATION_ID: AtomicU32 = AtomicU32::new(0);
impl TypedPersistentEntityId<Station> {
    pub fn next() -> Self {
        Self(NEXT_STATION_ID.fetch_add(1, Ordering::Relaxed), PhantomData)
    }
}
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
        write!(f, "[{}] {}", std::any::type_name::<T>(), self.0)
    }
}
impl<T: Component> Debug for TypedPersistentEntityId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
