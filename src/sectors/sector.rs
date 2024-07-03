use crate::sectors::sector_entity::SectorEntity;
use crate::sectors::{GateEntity, InSector};
use bevy::prelude::{Commands, Component, Entity, Vec2};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePair {
    pub from: GateEntity,
    pub to: GateEntity,
}

/// Marker Component for Sectors
#[derive(Component)]
pub struct Sector {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePair>,
    ships: HashSet<Entity>,
    stations: HashSet<Entity>,
}

impl Sector {
    pub fn new(coordinate: Hex, world_pos: Vec2) -> Self {
        Sector {
            coordinate,
            world_pos,
            gates: HashMap::new(),
            ships: HashSet::new(),
            stations: HashSet::new(),
        }
    }

    /// Adds ship to this sector and inserts the [InSector] component to it.
    pub fn add_ship(&mut self, commands: &mut Commands, sector: SectorEntity, entity: Entity) {
        self.ships.insert(entity);
        self.in_sector(commands, sector, entity);
    }

    /// Removes ship from this sector whilst also deleting its [InSector] component.
    pub fn remove_ship(&mut self, commands: &mut Commands, entity: Entity) {
        let result = self.ships.remove(&entity);
        debug_assert!(result, "removed ships should always be in sector!");
        commands.entity(entity).remove::<InSector>();
    }

    /// Adds the station to this sector and inserts the [InSector] component to it.
    pub fn add_station(&mut self, commands: &mut Commands, sector: SectorEntity, entity: Entity) {
        self.stations.insert(entity);
        self.in_sector(commands, sector, entity);
    }

    /// Adds the gate to this sector and inserts the [InSector] component to it.
    pub fn add_gate(
        &mut self,
        commands: &mut Commands,
        this_sector: SectorEntity,
        this_gate: GateEntity,
        destination_sector: SectorEntity,
        destination_gate: GateEntity,
    ) {
        self.gates.insert(
            destination_sector,
            GatePair {
                from: this_gate,
                to: destination_gate,
            },
        );
        self.in_sector(commands, this_sector, this_gate.get());
    }

    /// Adds the [InSector] component linking to `self` to the provided Entity.
    fn in_sector(&self, commands: &mut Commands, sector_entity: SectorEntity, entity: Entity) {
        commands.entity(entity).insert(InSector {
            sector: sector_entity,
        });
    }
}
