use crate::components::{ConstantOrbit, PlanetComponent, SelectableEntity};
use crate::persistence::{PlanetIdMap, PlanetKindSaveData, SectorPlanetSaveData};
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::ship_ai::AutoTradeBehavior;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::polar_coordinates::PolarCoordinates;
use crate::utils::{PlanetEntity, SectorEntity, SolarMass};
use crate::{SpriteHandles, components, constants};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Rot2};
use bevy::sprite::Sprite;

#[allow(clippy::too_many_arguments)]
pub fn spawn_planet(
    commands: &mut Commands,
    sector_planet_component: &mut components::SectorPlanetsComponent,
    planet_id_map: &mut PlanetIdMap,
    sprites: &SpriteHandles,
    planet_data: &SectorPlanetSaveData,
    sector_pos: Vec2,
    sector_entity: SectorEntity,
    orbit_mass: Option<SolarMass>,
) {
    let planet = PlanetComponent::new(planet_data.id, planet_data.mass);

    let simulation_transform =
        SimulationTransform::new(sector_pos + planet_data.local_position, Rot2::IDENTITY);

    let entity = commands
        .spawn((
            Name::new(planet_data.name.clone()),
            SelectableEntity::Planet,
            AutoTradeBehavior::default(),
            Sprite::from_image(sprites.planet.clone()),
            simulation_transform.as_bevy_transform(constants::z_layers::PLANET_AND_STARS),
            simulation_transform,
            SimulationScale::default(),
            planet,
        ))
        .id();

    if let Some(orbit_mass) = orbit_mass {
        let polar_coordinates = PolarCoordinates::from_cartesian(&planet_data.local_position);
        commands
            .entity(entity)
            .insert(ConstantOrbit::new(polar_coordinates, &orbit_mass));
    }

    match &planet_data.kind {
        PlanetKindSaveData::Terrestrial => {}
        PlanetKindSaveData::GasGiant { resources } => {
            commands.entity(entity).insert((
                components::GasGiant {
                    resources: resources.clone(),
                },
                components::InteractionQueue::new(constants::SIMULTANEOUS_PLANET_INTERACTIONS),
            ));
        }
    };

    let planet_entity = PlanetEntity::from(entity);
    planet_id_map.insert(planet_data.id, planet_entity);
    sector_planet_component.add_planet(commands, sector_entity, planet_entity);
}
