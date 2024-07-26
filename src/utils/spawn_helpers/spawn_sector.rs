use crate::components::{
    InSector, Sector, SectorAsteroidComponent, SectorStarComponent, SelectableEntity,
};
use crate::persistence::{PlanetIdMap, SectorFeatureSaveData};
use crate::simulation::asteroids::SectorWasSpawnedEvent;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::spawn_helpers::spawn_planet::spawn_planet;
use crate::utils::{SectorEntity, StarEntity};
use crate::{components, constants, SpriteHandles};
use bevy::core::Name;
use bevy::prelude::{Commands, EventWriter, Vec2};
use bevy::sprite::SpriteBundle;
use hexx::{Hex, HexLayout};

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    features: &SectorFeatureSaveData, // Create a feature list if we ever want to spawn sectors from something else than save data, but for now that's enough
    sector_spawn_event: &mut EventWriter<SectorWasSpawnedEvent>,
    sprites: &SpriteHandles,
    planet_id_map: &mut PlanetIdMap,
) -> SectorEntity {
    let position = layout.hex_to_world_pos(coordinate);

    let simulation_transform =
        SimulationTransform::from_translation(Vec2::new(position.x, position.y));

    let mut entity_commands = commands.spawn((
        Name::new(format!("[{},{}]", coordinate.x, coordinate.y)),
        Sector::new(coordinate, position),
        simulation_transform.as_transform(0.0),
        simulation_transform,
    ));

    if let Some(asteroids) = &features.asteroids {
        entity_commands.insert(SectorAsteroidComponent {
            // TODO
            average_velocity: asteroids.average_velocity,
            asteroids: Default::default(),
            asteroid_respawns: Default::default(),
        });
    }

    let sector_entity = entity_commands.id();
    let sector = SectorEntity::from(sector_entity);

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
                SpriteBundle {
                    transform: simulation_transform.as_transform(constants::PLANET_AND_STARS_LAYER),
                    texture: sprites.star.clone(),
                    ..Default::default()
                },
                simulation_transform,
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
            );
        }

        commands.entity(sector_entity).insert(component);
    }

    sector_spawn_event.send(SectorWasSpawnedEvent { sector });
    sector
}
