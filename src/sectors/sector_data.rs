use crate::sectors::InSector;
use crate::sectors::SectorId;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;

pub struct SectorData {
    pub id: SectorId,
    pub entity: Entity,
    pub world_pos: Vec2,
    pub gates: HashMap<SectorId, Entity>,
    ships: HashSet<Entity>,
    stations: HashSet<Entity>,
}

impl SectorData {
    pub fn new(coordinate: Hex, entity: Entity, world_pos: Vec2) -> Self {
        SectorData {
            id: coordinate,
            entity,
            world_pos,
            gates: HashMap::new(),
            ships: HashSet::new(),
            stations: HashSet::new(),
        }
    }

    /// Adds ship to this sector and inserts the [InSector] component to it.
    pub fn add_ship(&mut self, commands: &mut Commands, entity: Entity) {
        self.ships.insert(entity);
        commands.entity(entity).insert(InSector { sector: self.id });
    }

    /// Removes ship from this sector whilst also deleting its [InSector] component.
    pub fn remove_ship(&mut self, commands: &mut Commands, entity: Entity) {
        let result = self.ships.remove(&entity);
        debug_assert!(result, "removed ships should always be in sector!");

        commands.entity(entity).remove::<InSector>();
    }

    /// Adds the station to this sector and inserts the [InSector] component to it.
    pub fn add_station(&mut self, commands: &mut Commands, entity: Entity) {
        self.stations.insert(entity);
        commands.entity(entity).insert(InSector { sector: self.id });
    }
}
