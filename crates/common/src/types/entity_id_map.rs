use crate::types::entity_wrappers::{
    AsteroidEntity, CelestialEntity, ConstructionSiteEntity, GateEntity, SectorEntity, ShipEntity,
    StationEntity, TypedEntity,
};
use crate::types::persistent_entity_id::{
    PersistentAsteroidId, PersistentCelestialId, PersistentConstructionSiteId, PersistentEntityId,
    PersistentGateId, PersistentShipId, PersistentStationId,
};
use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Res, Resource};
use hexx::Hex;
use std::hash::Hash;

#[derive(SystemParam)]
pub struct AllEntityIdMaps<'w> {
    pub asteroids: Res<'w, AsteroidIdMap>,
    pub gates: Res<'w, GateIdMap>,
    pub celestials: Res<'w, CelestialIdMap>,
    pub sectors: Res<'w, SectorIdMap>,
    pub ships: Res<'w, ShipIdMap>,
    pub stations: Res<'w, StationIdMap>,
    pub build_sites: Res<'w, ConstructionSiteIdMap>,
}

impl AllEntityIdMaps<'_> {
    /// # Panics
    /// If no id is found for the given entity
    pub fn get_typed_id_unchecked(&self, typed_entity: &TypedEntity) -> PersistentEntityId {
        match typed_entity {
            TypedEntity::Asteroid(asteroid) => self.asteroids.entity_to_id[asteroid].into(),
            TypedEntity::ConstructionSite(build_site) => {
                self.build_sites.entity_to_id[build_site].into()
            }
            TypedEntity::Gate(gate) => self.gates.entity_to_id[gate].into(),
            TypedEntity::Celestial(celestial) => self.celestials.entity_to_id[celestial].into(),
            TypedEntity::Sector(sector) => self.sectors.entity_to_id[sector].into(),
            TypedEntity::Ship(ship) => self.ships.entity_to_id[ship].into(),
            TypedEntity::Station(station) => self.stations.entity_to_id[station].into(),
            TypedEntity::AnyWithInventory(entity_with_inventory) => {
                if let Some(id) = self.ships.get_id(&entity_with_inventory.into()) {
                    return id.into();
                }
                if let Some(id) = self.stations.get_id(&entity_with_inventory.into()) {
                    return id.into();
                }

                panic!()
            }
        }
    }
}

/// Maps [PersistentAsteroidId]s with the [AsteroidEntity]s they are representing.
pub type AsteroidIdMap = EntityIdMap<PersistentAsteroidId, AsteroidEntity>;

/// Maps [PersistentGateId]s with the [GateEntity]s they are representing.
pub type GateIdMap = EntityIdMap<PersistentGateId, GateEntity>;

/// Maps [PersistentCelestialId]s with the [CelestialEntity]s they are representing.
pub type CelestialIdMap = EntityIdMap<PersistentCelestialId, CelestialEntity>;

/// Maps [PersistentShipId]s with the [ShipEntity]s they are representing.
pub type ShipIdMap = EntityIdMap<PersistentShipId, ShipEntity>;

/// Maps [Hex]s with the [SectorEntity]s they are representing.
pub type SectorIdMap = EntityIdMap<Hex, SectorEntity>;

/// Maps [PersistentStationId]s with the [StationEntity]s they are representing.
pub type StationIdMap = EntityIdMap<PersistentStationId, StationEntity>;

/// Maps [PersistentConstructionSiteId]s with the [ConstructionSiteEntity]s they are representing.
pub type ConstructionSiteIdMap = EntityIdMap<PersistentConstructionSiteId, ConstructionSiteEntity>;

/// A simple Bidirectional Map.
///
/// If we ever need to do anything more complex than numeric id values, use the bimap crate.
#[derive(Resource)]
pub struct EntityIdMap<TId, TEntity>
where
    TId: Eq + Hash + Copy,
    TEntity: Eq + Hash + Copy,
{
    id_to_entity: HashMap<TId, TEntity>,
    entity_to_id: HashMap<TEntity, TId>,
}

impl<TId, TEntity> Default for EntityIdMap<TId, TEntity>
where
    TId: Eq + Hash + Copy,
    TEntity: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TId, TEntity> EntityIdMap<TId, TEntity>
where
    TId: Eq + Hash + Copy,
    TEntity: Eq + Hash + Copy,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            id_to_entity: HashMap::new(),
            entity_to_id: HashMap::new(),
        }
    }

    /// Read-Only access to the underlying id_to_entity HashMap.
    #[inline]
    pub fn entity_to_id(&self) -> &HashMap<TEntity, TId> {
        &self.entity_to_id
    }

    /// Read-Only access to the underlying entity_to_id HashMap.
    #[inline]
    pub fn id_to_entity(&self) -> &HashMap<TId, TEntity> {
        &self.id_to_entity
    }

    #[inline]
    pub fn insert(&mut self, id: TId, entity: TEntity) {
        self.id_to_entity.insert(id, entity);
        self.entity_to_id.insert(entity, id);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn remove_by_id(&mut self, id: &TId) {
        if let Some(entity) = self.id_to_entity.remove(id) {
            self.entity_to_id.remove(&entity);
        }
    }

    #[inline]
    pub fn remove_by_entity(&mut self, entity: &TEntity) {
        if let Some(id) = self.entity_to_id.remove(entity) {
            self.id_to_entity.remove(&id);
        }
    }

    #[inline]
    pub fn get_entity(&self, id: &TId) -> Option<&TEntity> {
        self.id_to_entity.get(id)
    }

    #[inline]
    pub fn get_id(&self, entity: &TEntity) -> Option<&TId> {
        self.entity_to_id.get(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Entity;

    #[test]
    fn inserting_and_getting_data() {
        let id1 = PersistentGateId::next();
        let id2 = PersistentGateId::next();

        let entity1 = GateEntity::from(Entity::from_raw(1));
        let entity2 = GateEntity::from(Entity::from_raw(2));

        let mut map = GateIdMap::new();
        map.insert(id1, entity1);
        map.insert(id2, entity2);

        assert_eq!(&id1, map.get_id(&entity1).unwrap());
        assert_eq!(&id2, map.get_id(&entity2).unwrap());

        assert_eq!(&entity1, map.get_entity(&id1).unwrap());
        assert_eq!(&entity2, map.get_entity(&id2).unwrap());
    }

    #[test]
    fn removing_data() {
        let id1 = PersistentGateId::next();
        let id2 = PersistentGateId::next();

        let entity1 = GateEntity::from(Entity::from_raw(1));
        let entity2 = GateEntity::from(Entity::from_raw(2));

        let mut map = GateIdMap::new();
        map.insert(id1, entity1);
        map.insert(id2, entity2);

        map.remove_by_id(&id1);

        assert_eq!(1, map.id_to_entity.len());
        assert_eq!(1, map.entity_to_id.len());

        assert_eq!(None, map.get_entity(&id1));
        assert_eq!(None, map.get_id(&entity1));

        assert_eq!(&entity2, map.get_entity(&id2).unwrap());
        assert_eq!(&id2, map.get_id(&entity2).unwrap());
    }
}
