use crate::components::InSector;
use crate::utils::{PlanetEntity, SectorEntity};
use bevy::prelude::{Commands, Component};
use bevy::utils::HashSet;

/// A sector with this component features planets.
#[derive(Component)]
pub struct SectorPlanets {
    pub planets: HashSet<PlanetEntity>,
}

impl SectorPlanets {
    pub fn new() -> Self {
        Self {
            planets: Default::default(),
        }
    }

    /// Adds the given `planet_entity` and inserts the [InSector] component to it.
    pub fn add_planet(
        &mut self,
        commands: &mut Commands,
        sector_entity: SectorEntity,
        planet_entity: PlanetEntity,
    ) {
        self.planets.insert(planet_entity);
        InSector::add_component(commands, sector_entity, planet_entity.into());
    }
}
