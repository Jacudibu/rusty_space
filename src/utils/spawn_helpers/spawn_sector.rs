use crate::components::{Sector, SectorAsteroidData};
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
    asteroids: Option<SectorAsteroidData>,
    sector_spawn_event: &mut EventWriter<SectorWasSpawnedEvent>,
) -> SectorEntity {
    let position = layout.hex_to_world_pos(coordinate);

    let simulation_transform =
        SimulationTransform::from_translation(Vec2::new(position.x, position.y));

    let entity = commands
        .spawn((
            Name::new(format!("[{},{}]", coordinate.x, coordinate.y)),
            Sector::new(coordinate, position, asteroids),
            simulation_transform.as_transform(0.0),
            simulation_transform,
        ))
        .id();

    let sector = SectorEntity::from(entity);
    sector_spawn_event.send(SectorWasSpawnedEvent { sector });
    sector
}
