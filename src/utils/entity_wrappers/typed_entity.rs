use crate::utils::{
    AsteroidEntity, GateEntity, PlanetEntity, SectorEntity, ShipEntity, StarEntity, StationEntity,
};
use bevy::prelude::Entity;

#[derive(Copy, Clone, Debug)]
pub enum TypedEntity {
    Asteroid(AsteroidEntity),
    Gate(GateEntity),
    Planet(PlanetEntity),
    Sector(SectorEntity),
    Ship(ShipEntity),
    Star(StarEntity),
    Station(StationEntity),
    AnyWithInventory(Entity),
}

impl From<TypedEntity> for Entity {
    fn from(value: TypedEntity) -> Self {
        match value {
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Planet(inner) => inner.into(),
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Star(inner) => inner.into(),
            TypedEntity::Station(inner) => inner.into(),
            TypedEntity::AnyWithInventory(inner) => inner,
        }
    }
}

impl From<&TypedEntity> for Entity {
    fn from(value: &TypedEntity) -> Self {
        match value {
            TypedEntity::Asteroid(inner) => inner.into(),
            TypedEntity::Gate(inner) => inner.into(),
            TypedEntity::Planet(inner) => inner.into(),
            TypedEntity::Sector(inner) => inner.into(),
            TypedEntity::Ship(inner) => inner.into(),
            TypedEntity::Star(inner) => inner.into(),
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

impl From<GateEntity> for TypedEntity {
    fn from(value: GateEntity) -> Self {
        Self::Gate(value)
    }
}

impl From<PlanetEntity> for TypedEntity {
    fn from(value: PlanetEntity) -> Self {
        Self::Planet(value)
    }
}

impl From<ShipEntity> for TypedEntity {
    fn from(value: ShipEntity) -> Self {
        Self::Ship(value)
    }
}

impl From<StarEntity> for TypedEntity {
    fn from(value: StarEntity) -> Self {
        Self::Star(value)
    }
}

impl From<StationEntity> for TypedEntity {
    fn from(value: StationEntity) -> Self {
        Self::Station(value)
    }
}
