use crate::components::{BuildSite, InSector};
use crate::utils::{GateEntity, SectorEntity, ShipEntity, StationEntity};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component};
use bevy::utils::{HashMap, HashSet};
use hexx::Hex;

/// Marker Component for Sectors
#[derive(Component)]
pub struct Sector {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePairInSector>,
    pub ships: HashSet<ShipEntity>,
    pub stations: HashSet<StationEntity>,
    pub build_sites: HashSet<BuildSite>,
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
            ships: HashSet::new(),
            stations: HashSet::new(),
            build_sites: HashSet::new(),
        }
    }

    /// Adds the given ship to this sector and inserts the [InSector] component to it.
    pub fn add_ship(&mut self, commands: &mut Commands, sector: SectorEntity, entity: ShipEntity) {
        self.ships.insert(entity);
        InSector::add_component(commands, sector, entity.into());
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
        InSector::add_component(commands, sector, entity.into());
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
        InSector::add_component(commands, this_sector, this_gate.into());
    }
}
