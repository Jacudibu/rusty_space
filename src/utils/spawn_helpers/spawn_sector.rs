use crate::components::{Sector, SectorAsteroidComponent, SectorAsteroidData, SectorStarComponent};
use crate::persistence::SectorFeatureSaveData;
use crate::simulation::asteroids::SectorWasSpawnedEvent;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::SectorEntity;
use bevy::core::Name;
use bevy::prelude::{Commands, EventWriter, Vec2};
use hexx::{Hex, HexLayout};

pub fn spawn_sector(
    commands: &mut Commands,
    layout: &HexLayout,
    coordinate: Hex,
    features: &SectorFeatureSaveData, // Create a feature list if we ever want to spawn sectors from something else than save data, but for now that's enough
    sector_spawn_event: &mut EventWriter<SectorWasSpawnedEvent>,
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

    if let Some(star) = &features.star {
        entity_commands.insert(SectorStarComponent { mass: star.mass });
    }
    if let Some(asteroids) = &features.asteroids {
        entity_commands.insert(SectorAsteroidComponent {
            // TODO
            asteroids: Default::default(),
            asteroid_respawns: Default::default(),
            asteroid_data: SectorAsteroidData::from(asteroids),
        });
    }

    let entity = entity_commands.id();
    let sector = SectorEntity::from(entity);
    sector_spawn_event.send(SectorWasSpawnedEvent { sector });
    sector
}
