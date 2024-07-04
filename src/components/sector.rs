use crate::components::InSector;
use crate::utils::{
    AsteroidEntityWithLifetime, GateEntity, SectorEntity, ShipEntity, StationEntity,
};
use bevy::prelude::{Commands, Component, Entity, Vec2};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;
use std::collections::BinaryHeap;

/// Marker Component for Sectors
#[derive(Component)]
pub struct Sector {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePairInSector>,
    ships: HashSet<ShipEntity>,
    stations: HashSet<StationEntity>,

    pub asteroid_data: Option<SectorAsteroidData>,
    pub asteroids: BinaryHeap<AsteroidEntityWithLifetime>,
}

#[derive(Copy, Clone)]
pub struct SectorAsteroidData {
    // All asteroids within a Sector behave the same, or maybe later on a range.
    pub forward_velocity: f32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePairInSector {
    /// The gate inside this sector
    pub from: GateEntity,

    /// The gate it's connected to
    pub to: GateEntity,
}

impl Sector {
    pub fn new(coordinate: Hex, world_pos: Vec2, asteroids: Option<SectorAsteroidData>) -> Self {
        Sector {
            coordinate,
            world_pos,
            asteroid_data: asteroids,
            asteroids: BinaryHeap::new(),
            gates: HashMap::new(),
            ships: HashSet::new(),
            stations: HashSet::new(),
        }
    }

    /// Adds asteroid to this sector and inserts the [InSector] component to it.
    pub fn add_asteroid(
        &mut self,
        commands: &mut Commands,
        sector: SectorEntity,
        entity: AsteroidEntityWithLifetime,
    ) {
        self.add_asteroid_in_place(entity);
        self.in_sector(commands, sector, entity.entity.into());
    }

    /// Adds asteroid to this sectors' asteroid set.
    pub fn add_asteroid_in_place(&mut self, entity: AsteroidEntityWithLifetime) {
        self.asteroids.push(entity);
    }

    /// Adds ship to this sector and inserts the [InSector] component to it.
    pub fn add_ship(&mut self, commands: &mut Commands, sector: SectorEntity, entity: ShipEntity) {
        self.ships.insert(entity);
        self.in_sector(commands, sector, entity.into());
    }

    /// Removes ship from this sector whilst also deleting its [InSector] component.
    pub fn remove_ship(&mut self, commands: &mut Commands, entity: ShipEntity) {
        let result = self.ships.remove(&entity);
        debug_assert!(result, "removed ships should always be in sector!");
        commands.entity(entity.into()).remove::<InSector>();
    }

    /// Adds the station to this sector and inserts the [InSector] component to it.
    pub fn add_station(
        &mut self,
        commands: &mut Commands,
        sector: SectorEntity,
        entity: StationEntity,
    ) {
        self.stations.insert(entity);
        self.in_sector(commands, sector, entity.into());
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
            GatePairInSector {
                from: this_gate,
                to: destination_gate,
            },
        );
        self.in_sector(commands, this_sector, this_gate.into());
    }

    /// Adds the [InSector] component linking to `self` to the provided Entity.
    fn in_sector(&self, commands: &mut Commands, sector_entity: SectorEntity, entity: Entity) {
        commands.entity(entity).insert(InSector {
            sector: sector_entity,
        });
    }
}
