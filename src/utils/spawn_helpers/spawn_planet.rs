use crate::components::{ConstantOrbit, Planet, SelectableEntity};
use crate::persistence::{PlanetIdMap, PlanetKindSaveData, SectorPlanetSaveData};
use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::ship_ai::AutoTradeBehavior;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::spawn_helpers::helpers;
use crate::utils::{PlanetEntity, SectorEntity, SolarMass};
use crate::{components, constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Rot2, SpriteBundle};

#[allow(clippy::too_many_arguments)]
pub fn spawn_planet(
    commands: &mut Commands,
    sector_planet_component: &mut components::SectorPlanets,
    planet_id_map: &mut PlanetIdMap,
    sprites: &SpriteHandles,
    planet_data: &SectorPlanetSaveData,
    sector_pos: Vec2,
    sector_entity: SectorEntity,
    orbit_mass: Option<SolarMass>,
    orbit_directions: &PrecomputedOrbitDirections,
) {
    let planet = Planet::new(planet_data.id, planet_data.mass);

    let local_position = orbit_directions.orbit_position_at(
        planet_data.orbit.radius,
        planet_data.orbit.current_rotational_fraction,
    );

    let velocity = if let Some(orbit_mass) = orbit_mass {
        helpers::calculate_orbit_velocity(planet_data.orbit.radius, orbit_mass)
    } else {
        0.0
    };

    let simulation_transform =
        SimulationTransform::new(sector_pos + local_position, Rot2::IDENTITY);

    let entity = commands
        .spawn((
            Name::new(planet_data.name.clone()),
            SelectableEntity::Planet,
            AutoTradeBehavior::default(),
            ConstantOrbit::new(
                planet_data.orbit.current_rotational_fraction,
                planet_data.orbit.radius,
                velocity,
            ),
            SpriteBundle {
                texture: sprites.planet.clone(),
                transform: simulation_transform.as_transform(constants::z_layers::PLANET_AND_STARS),
                ..default()
            },
            simulation_transform,
            SimulationScale::default(),
            planet,
        ))
        .id();

    match planet_data.kind {
        PlanetKindSaveData::Terrestrial => {}
        PlanetKindSaveData::GasGiant => {
            commands.entity(entity).insert((
                components::GasGiant {},
                components::InteractionQueue::new(constants::SIMULTANEOUS_PLANET_INTERACTIONS),
            ));
        }
    };

    let planet_entity = PlanetEntity::from(entity);
    planet_id_map.insert(planet_data.id, planet_entity);
    sector_planet_component.add_planet(commands, sector_entity, planet_entity);
}
