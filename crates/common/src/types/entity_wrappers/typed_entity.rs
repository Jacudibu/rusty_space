use crate::types::entity_wrappers::{
    AsteroidEntity, CelestialEntity, ConstructionSiteEntity, GateEntity, SectorEntity, ShipEntity,
    StationEntity,
};
use bevy::prelude::Entity;

/// An enum-wrapper around entities to ensure type safety for mutually exclusive component combinations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypedEntity {
    Asteroid(AsteroidEntity),
    Celestial(CelestialEntity),
    ConstructionSite(ConstructionSiteEntity),
    Gate(GateEntity),
    Sector(SectorEntity),
    Ship(ShipEntity),
    Station(StationEntity),
    AnyWithInventory(Entity),
}

impl From<TypedEntity> for Entity {
    fn from(value: TypedEntity) -> Self {
        match value {
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::Celestial(inner) => inner.into(),
            TypedEntity::ConstructionSite(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Station(inner) => inner.into(),
            TypedEntity::AnyWithInventory(inner) => inner,
        }
    }
}

impl From<&TypedEntity> for Entity {
    fn from(value: &TypedEntity) -> Self {
        match value {
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::Celestial(inner) => inner.into(),
            TypedEntity::ConstructionSite(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Station(inner) => inner.into(),
            TypedEntity::AnyWithInventory(inner) => *inner,
        }
    }
}

impl From<AsteroidEntity> for TypedEntity {
    fn from(value: AsteroidEntity) -> Self {
        Self::Asteroid(value)
    }
}

impl From<CelestialEntity> for TypedEntity {
    fn from(value: CelestialEntity) -> Self {
        Self::Celestial(value)
    }
}

impl From<GateEntity> for TypedEntity {
    fn from(value: GateEntity) -> Self {
        Self::Gate(value)
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

impl From<ConstructionSiteEntity> for TypedEntity {
    fn from(value: ConstructionSiteEntity) -> Self {
        Self::ConstructionSite(value)
    }
}
