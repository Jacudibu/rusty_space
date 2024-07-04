use crate::asteroid_system::SectorWasSpawnedEvent;
use crate::components::SectorAsteroidData;
use crate::map_layout::MapLayout;
use crate::test_universe::plugin::TestSectors;
use crate::utils::spawn_helpers::spawn_sector;
use bevy::prelude::{Commands, EventWriter, Res};
use hexx::Hex;

pub fn spawn_test_sectors(
    mut commands: Commands,
    map_layout: Res<MapLayout>,
    mut sector_spawn_event: EventWriter<SectorWasSpawnedEvent>,
) {
    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let top_right = Hex::new(0, 1);
    let bottom_left = Hex::new(0, -1);

    let asteroids = Some(SectorAsteroidData {
        forward_velocity: 10.0,
        angular_velocity: 0.001,
    });

    let center_sector = spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        center,
        None,
        &mut sector_spawn_event,
    );
    let right_sector = spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        right,
        None,
        &mut sector_spawn_event,
    );
    let top_right_sector = spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        top_right,
        asteroids,
        &mut sector_spawn_event,
    );
    let bottom_left_sector = spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        bottom_left,
        None,
        &mut sector_spawn_event,
    );

    commands.insert_resource(TestSectors {
        center: center_sector,
        right: right_sector,
        top_right: top_right_sector,
        bottom_left: bottom_left_sector,
    });
}
