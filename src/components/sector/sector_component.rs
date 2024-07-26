use crate::components::InSector;
use crate::utils::{GateEntity, PlanetEntity, SectorEntity, ShipEntity, StationEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;

/// Marker Component for Sectors
#[derive(Component)]
pub struct Sector {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePairInSector>,
    pub planets: HashSet<PlanetEntity>,
    pub ships: HashSet<ShipEntity>,
    pub stations: HashSet<StationEntity>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePairInSector {
    /// The gate inside this sector
    pub from: GateEntity,

    /// The gate it's connected to
    pub to: GateEntity,
}

impl Sector {
    pub fn new(coordinate: Hex, world_pos: Vec2) -> Self {
        Sector {
            coordinate,
            world_pos,
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
    pub fn in_sector(commands: &mut Commands, sector_entity: SectorEntity, entity: Entity) {
        commands.entity(entity).insert(InSector {
            sector: sector_entity,
        });
    }
}
