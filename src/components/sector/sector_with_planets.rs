use crate::components::InSector;
use crate::utils::{PlanetEntity, SectorEntity};
use bevy::platform::collections::HashSet;
use bevy::prelude::{Commands, Component};

/// A [Sector] with this component features planets.
#[derive(Component)]
#[component(immutable)]
pub struct SectorWithPlanets {
    pub planets: HashSet<PlanetEntity>,
}

impl SectorWithPlanets {
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
