use crate::components::InSector;
use crate::utils::{
    AsteroidEntityWithTimestamp, GateEntity, PlanetEntity, SectorEntity, ShipEntity, StationEntity,
};
use bevy::prelude::{Commands, Component, Entity, Vec2};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;
use std::collections::{BTreeSet, BinaryHeap};

/// Marker Component for Sectors
#[derive(Component)]
pub struct Sector {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePairInSector>,
    pub planets: HashSet<PlanetEntity>,
    pub ships: HashSet<ShipEntity>,
    pub feature: SectorFeature,
    pub stations: HashSet<StationEntity>,
}

/// The main feature of a sector.
pub enum SectorFeature {
    /// The sector is devoid of any natural objects
    Void,

    /// The sector features a star. Planets, Gates (?) and Stations (?) orbit around that.
    Star,

    // TODO: Just an idea - contemplate using this over asteroid_data and asteroids, since asteroids moving in orbit would be a headache
    /// The sector features asteroids, idly floating through it.
    Asteroids(AsteroidFeature),
}

impl SectorFeature {
    pub fn is_asteroids(&self) -> bool {
        match self {
            SectorFeature::Void => false,
            SectorFeature::Star => false,
            SectorFeature::Asteroids(_) => true,
        }
    }
}

pub struct AsteroidFeature {
    pub asteroid_data: SectorAsteroidData,
    pub asteroids: BTreeSet<AsteroidEntityWithTimestamp>,
    pub asteroid_respawns: BinaryHeap<std::cmp::Reverse<AsteroidEntityWithTimestamp>>,
}

impl AsteroidFeature {
    pub fn new(asteroid_data: SectorAsteroidData) -> Self {
        Self {
            asteroid_data,
            asteroids: BTreeSet::new(),
            asteroid_respawns: BinaryHeap::new(),
        }
    }

    /// Adds the given asteroid to this sector and inserts the [InSector] component to it.
    pub fn add_asteroid(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: AsteroidEntityWithTimestamp,
    ) {
        self.add_asteroid_in_place(entity);
        Sector::in_sector(commands, sector_entity, entity.entity.into());
    }

    /// Adds asteroid to this sectors' asteroid set.
    pub fn add_asteroid_in_place(&mut self, entity: AsteroidEntityWithTimestamp) {
        self.asteroids.insert(entity);
    }
}

#[derive(Copy, Clone)]
pub struct SectorAsteroidData {
    // All asteroids within a Sector behave the same, or maybe later on a range.
    pub average_velocity: Vec2,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePairInSector {
    /// The gate inside this sector
    pub from: GateEntity,

    /// The gate it's connected to
    pub to: GateEntity,
}

impl Sector {
    pub fn new(coordinate: Hex, world_pos: Vec2, feature: SectorFeature) -> Self {
        Sector {
            coordinate,
            world_pos,
            feature,
            gates: HashMap::new(),
            planets: HashSet::new(),
            ships: HashSet::new(),
            stations: HashSet::new(),
        }
    }

    /// Adds the given planet to this sector and inserts the [InSector] component to it.
    pub fn add_planet(
        &mut self,
        commands: &mut Commands,
        sector: SectorEntity,
        entity: PlanetEntity,
    ) {
        self.planets.insert(entity);
        Self::in_sector(commands, sector, entity.into());
    }

    /// Adds the given ship to this sector and inserts the [InSector] component to it.
    pub fn add_ship(&mut self, commands: &mut Commands, sector: SectorEntity, entity: ShipEntity) {
        self.ships.insert(entity);
        Self::in_sector(commands, sector, entity.into());
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
        Self::in_sector(commands, sector, entity.into());
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
        Self::in_sector(commands, this_sector, this_gate.into());
    }

    /// Adds the [InSector] component linking to `self` to the provided Entity.
    fn in_sector(commands: &mut Commands, sector_entity: SectorEntity, entity: Entity) {
        commands.entity(entity).insert(InSector {
            sector: sector_entity,
        });
    }
}
