use crate::components::{
    InSector, Sector, SectorAsteroidComponent, SectorStarComponent, SelectableEntity,
};
use crate::game_data::AsteroidManifest;
use crate::persistence::{AsteroidIdMap, PlanetIdMap, SectorFeatureSaveData};
use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::entity_spawners::spawn_planet::spawn_planet;
use crate::utils::{entity_spawners, SectorEntity, StarEntity};
use crate::{components, constants, SpriteHandles};
use bevy::core::Name;
use bevy::prelude::{Commands, Sprite, Vec2};
use hexx::{Hex, HexLayout};

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    features: &SectorFeatureSaveData, // Create a feature list if we ever want to spawn sectors from something else than save data, but for now that's enough
    sprites: &SpriteHandles,
    asteroid_id_map: &mut AsteroidIdMap,
    planet_id_map: &mut PlanetIdMap,
    orbit_directions: &PrecomputedOrbitDirections,
    asteroid_manifest: &AsteroidManifest,
) -> SectorEntity {
    let position = layout.hex_to_world_pos(coordinate);

    let simulation_transform =
        SimulationTransform::from_translation(Vec2::new(position.x, position.y));

    let entity_commands = commands.spawn((
        Name::new(format!("[{},{}]", coordinate.x, coordinate.y)),
        Sector::new(coordinate, position),
        simulation_transform.as_bevy_transform(0.0),
        simulation_transform,
    ));

    let sector_entity = entity_commands.id();
    let sector = SectorEntity::from(sector_entity);

    if let Some(asteroids) = &features.asteroids {
        let mut component = SectorAsteroidComponent::new(
            asteroids.average_velocity,
            asteroids.asteroid_materials.clone(),
        );

        for x in &asteroids.live_asteroids {
            entity_spawners::spawn_asteroid(
                commands,
                asteroid_id_map,
                x.manifest_id,
                asteroid_manifest,
                x.position,
                &mut component,
                sector,
                x.velocity,
                x.ore_current,
                x.ore_max,
                x.rotation_degrees,
                x.angular_velocity,
                x.lifetime,
                false,
            );
        }

        commands.entity(sector_entity).insert(component);
    }

    let mut gravitation_well_mass = None;
    if let Some(star) = &features.star {
        gravitation_well_mass = Some(star.mass);
        let simulation_transform = SimulationTransform::from_translation(position);

        let star_entity = commands
            .spawn((
                Name::new(format!("[{},{}] Star", coordinate.x, coordinate.y)),
                components::Star::new(coordinate, star.mass),
                InSector { sector },
                SelectableEntity::Star,
                Sprite::from_image(sprites.star.clone()),
                simulation_transform.as_bevy_transform(constants::z_layers::PLANET_AND_STARS),
                simulation_transform,
                SimulationScale::default(),
            ))
            .id();

        commands.entity(sector_entity).insert(SectorStarComponent {
            entity: StarEntity::from(star_entity),
        });
    }

    if let Some(planets) = &features.planets {
        let mut component = components::SectorPlanets {
            planets: Default::default(),
        };

        for planet in planets {
            spawn_planet(
                commands,
                &mut component,
                planet_id_map,
                sprites,
                planet,
                position,
                sector,
                gravitation_well_mass,
                orbit_directions,
            );
        }

        commands.entity(sector_entity).insert(component);
    }

    sector
}
