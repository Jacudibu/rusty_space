use crate::persistence::persistent_entity_id::{
    PersistentAsteroidId, PersistentGateId, PersistentShipId, PersistentStationId,
};
use crate::persistence::PersistentEntityId;
use crate::utils::{
    AsteroidEntity, GateEntity, SectorEntity, ShipEntity, StationEntity, TypedEntity,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, Resource};
use bevy::utils::HashMap;
use hexx::Hex;
use std::hash::Hash;

#[derive(SystemParam)]
pub struct AllEntityIdMaps<'w> {
    pub asteroids: Res<'w, AsteroidIdMap>,
    pub gates: Res<'w, GateIdMap>,
    pub sectors: Res<'w, SectorIdMap>,
    pub ships: Res<'w, ShipIdMap>,
    pub stations: Res<'w, StationIdMap>,
}

impl<'w> AllEntityIdMaps<'w> {
    /// # Panics
    /// Panics if no id is found for the given entity
    pub fn get_typed_id_unchecked(&self, typed_entity: &TypedEntity) -> PersistentEntityId {
        match typed_entity {
            TypedEntity::Asteroid(asteroid) => self.asteroids.entity_to_id[asteroid].into(),
            TypedEntity::Gate(gate) => self.gates.entity_to_id[gate].into(),
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

pub type AsteroidIdMap = EntityIdMap<PersistentAsteroidId, AsteroidEntity>;
pub type GateIdMap = EntityIdMap<PersistentGateId, GateEntity>;
pub type ShipIdMap = EntityIdMap<PersistentShipId, ShipEntity>;
pub type StationIdMap = EntityIdMap<PersistentStationId, StationEntity>;
pub type SectorIdMap = EntityIdMap<Hex, SectorEntity>;

/// A simple Bidirectional Map.
///
/// For anything more complex than numeric id values, use the bimap crate.
#[derive(Resource)]
pub struct EntityIdMap<TId, TEntity>
where
    TId: Eq + Hash + Copy,
    TEntity: Eq + Hash + Copy,
{
    id_to_entity: HashMap<TId, TEntity>,
    entity_to_id: HashMap<TEntity, TId>,
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
    fn inserting_getting_data() {
        let id1 = PersistentGateId::default();
        let id2 = PersistentGateId::default();

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
}
