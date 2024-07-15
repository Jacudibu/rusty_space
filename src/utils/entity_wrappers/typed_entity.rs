use crate::utils::{AsteroidEntity, GateEntity, SectorEntity, ShipEntity, StationEntity};
use bevy::prelude::Entity;

#[derive(Copy, Clone, Debug)]
pub enum TypedEntity {
    Sector(SectorEntity),
    Gate(GateEntity),
    Ship(ShipEntity),
    Station(StationEntity),
    Asteroid(AsteroidEntity),
    AnyWithInventory(Entity),
}

impl From<TypedEntity> for Entity {
    fn from(value: TypedEntity) -> Self {
        match value {
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Station(inner) => inner.into(),
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::AnyWithInventory(inner) => inner,
        }
    }
}

impl From<&TypedEntity> for Entity {
    fn from(value: &TypedEntity) -> Self {
        match value {
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Station(inner) => inner.into(),
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::AnyWithInventory(inner) => *inner,
        }
    }
}

impl From<GateEntity> for TypedEntity {
    fn from(value: GateEntity) -> Self {
        Self::Gate(value)
    }
}

impl From<AsteroidEntity> for TypedEntity {
    fn from(value: AsteroidEntity) -> Self {
        Self::Asteroid(value)
    }
}

impl From<ShipEntity> for TypedEntity {
    fn from(value: ShipEntity) -> Self {
        Self::Ship(value)
    }
}

impl From<StationEntity> for TypedEntity {
    fn from(value: StationEntity) -> Self {
        Self::Station(value)
    }
}
