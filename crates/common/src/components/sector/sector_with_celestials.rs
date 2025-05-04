use crate::components::InSector;
use crate::enums::celestial_mass::CelestialMass;
use crate::types::entity_wrappers::{CelestialEntity, SectorEntity};
use bevy::prelude::{Commands, Component};
use std::collections::HashSet;

/// Marker Component for a [Sector] featuring celestials and orbit mechanics.
#[derive(Component)]
#[component(immutable)]
pub struct SectorWithCelestials {
    /// The mass which resides at the center of this sector and is used to calculate orbit mechanics.
    /// Usually that's a star, but... you never know!~
    pub center_mass: CelestialMass,

    /// All stars which can be found inside this sector
    pub stars: HashSet<CelestialEntity>,
    /// All planets that can be found within this sector
    pub planets: HashSet<CelestialEntity>,
    /// All gas giants that can be found within this sector
    pub gas_giants: HashSet<CelestialEntity>,
}

impl SectorWithCelestials {
    /// Creates a new instance of [SectorWithCelestials] with sensible defaults.
    pub fn new(center_mass: CelestialMass) -> Self {
        Self {
            center_mass,
            stars: HashSet::new(),
            planets: HashSet::new(),
            gas_giants: HashSet::new(),
        }
    }

    /// Adds the given [Planet] entity and inserts the [InSector] component to it.
    pub fn add_planet(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: CelestialEntity,
    ) {
        self.planets.insert(entity);
        InSector::add_component(commands, sector_entity, entity.into());
    }

    /// Adds the given [GasGiant] entity and inserts the [InSector] component to it.
    pub fn add_gas_giant(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: CelestialEntity,
    ) {
        self.gas_giants.insert(entity);
        InSector::add_component(commands, sector_entity, entity.into());
    }

    /// Adds the given [Star] entity and inserts the [InSector] component to it.
    pub fn add_star(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        entity: CelestialEntity,
    ) {
        self.stars.insert(entity);
        InSector::add_component(commands, sector_entity, entity.into());
    }
}
