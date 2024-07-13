use crate::asteroids::SectorWasSpawnedEvent;
use crate::map_layout::MapLayout;
use crate::persistence::SectorIdMap;
use crate::universe_builder::sector_builder::data_resource::SectorSpawnData;
use bevy::prelude::{Commands, EventWriter, Res};

pub fn spawn_all_sectors(
    mut commands: Commands,
    map_layout: Res<MapLayout>,
    spawn_data: Res<SectorSpawnData>,
    mut sector_spawn_event: EventWriter<SectorWasSpawnedEvent>,
) {
    let mut entity_map = SectorIdMap::new();
    for builder in &spawn_data.sectors {
        let entity = builder.build(
            &mut commands,
            &map_layout.hex_layout,
            &mut sector_spawn_event,
        );
        entity_map.insert(builder.coordinate, entity);
    }

    commands.remove_resource::<SectorSpawnData>();
    commands.insert_resource(entity_map);
}
