use crate::SpriteHandles;
use crate::persistence::{AsteroidIdMap, CelestialIdMap, SectorFeatureSaveData};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::entity_spawners::spawn_planet::spawn_celestial;
use crate::utils::{SectorEntity, entity_spawners};
use bevy::prelude::{Commands, Name, Vec2};
use common::components::{Sector, SectorWithAsteroids, SectorWithCelestials};
use common::game_data::AsteroidManifest;
use hexx::{Hex, HexLayout};

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    features: &SectorFeatureSaveData, // Create a feature list if we ever want to spawn sectors from something else than save data, but for now that's enough
    sprites: &SpriteHandles,
    asteroid_id_map: &mut AsteroidIdMap,
    celestial_id_map: &mut CelestialIdMap,
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
        let mut component = SectorWithAsteroids::new(
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

    if let Some(celestials) = &features.celestials {
        let center_mass = Some(celestials.center_mass);
        let mut component = SectorWithCelestials::new(celestials.center_mass);

        for celestial_data in &celestials.celestials {
            spawn_celestial(
                commands,
                &mut component,
                celestial_id_map,
                sprites,
                celestial_data,
                position,
                sector,
                center_mass,
            );
        }

        commands.entity(sector_entity).insert(component);
    }

    sector
}
