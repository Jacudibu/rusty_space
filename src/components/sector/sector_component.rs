use crate::components::InSector;
use crate::utils::{ConstructionSiteEntity, GateEntity, SectorEntity, ShipEntity, StationEntity};
use bevy::math::Vec2;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::{Commands, Component};
use hexx::Hex;

/// Main Component for Sector Entities. Keeps track of common entities which can be found inside all sectors.
#[derive(Component)]
pub struct SectorComponent {
    pub coordinate: Hex,
    pub world_pos: Vec2,

    pub gates: HashMap<SectorEntity, GatePairInSector>,
    pub ships: HashSet<ShipEntity>,
    pub stations: HashSet<StationEntity>,
    pub construction_sites: HashSet<ConstructionSiteEntity>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GatePairInSector {
    /// The gate inside this sector
    pub from: GateEntity,

    /// The gate it's connected to
    pub to: GateEntity,
}

impl SectorComponent {
    pub fn new(coordinate: Hex, world_pos: Vec2) -> Self {
        SectorComponent {
            coordinate,
            world_pos,
            gates: HashMap::new(),
            ships: HashSet::new(),
            stations: HashSet::new(),
            construction_sites: HashSet::new(),
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

    /// Adds the construction site to this sector and inserts the [InSector] component to it.
    pub fn add_construction_site(
        &mut self,
        commands: &mut Commands,
        sector: SectorEntity,
        entity: ConstructionSiteEntity,
    ) {
        self.construction_sites.insert(entity);
        InSector::add_component(commands, sector, entity.into());
    }

    /// Removes a construction site from this Sector. It must have either been destroyed or completed, so we don't bother touching the [InSector] component.
    pub fn remove_construction_site(&mut self, entity: ConstructionSiteEntity) {
        let result = self.construction_sites.remove(&entity);
        debug_assert!(result, "removed con-sites should always be in sector!");
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
