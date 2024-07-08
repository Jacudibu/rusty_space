use crate::asteroid_system::SectorWasSpawnedEvent;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::map_layout::MapLayout;
use crate::universe_builder::sector_builder::resources::SectorSpawnData;
use bevy::prelude::{Commands, EventWriter, Res};
use bevy::utils::HashMap;

pub fn spawn_all_sectors(
    mut commands: Commands,
    map_layout: Res<MapLayout>,
    spawn_data: Res<SectorSpawnData>,
    mut sector_spawn_event: EventWriter<SectorWasSpawnedEvent>,
) {
    let mut entity_map = HashMap::new();
    for builder in &spawn_data.sectors {
        let entity = builder.build(
            &mut commands,
            &map_layout.hex_layout,
            &mut sector_spawn_event,
        );
        entity_map.insert(builder.coordinate, entity);
    }

    commands.remove_resource::<SectorSpawnData>();
    commands.insert_resource(HexToSectorEntityMap { map: entity_map });
}
