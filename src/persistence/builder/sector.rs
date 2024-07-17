use crate::asteroids::SectorWasSpawnedEvent;
use crate::components::SectorAsteroidData;
use crate::map_layout::MapLayout;
use crate::persistence::data::v1::{SaveDataCollection, SectorAsteroidSaveData, SectorSaveData};
use crate::persistence::{AsteroidIdMap, SectorIdMap};
use crate::utils::{spawn_helpers, SectorEntity};
use bevy::prelude::{Commands, EventWriter, Res};
use hexx::Hex;

impl SaveDataCollection<SectorSaveData> {
    pub fn add(&mut self, hex: Hex) -> &mut SectorSaveData {
        self.data.push(SectorSaveData {
            coordinate: hex,
            asteroid_data: None,
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        });
        self.data.last_mut().unwrap()
    }

    pub fn spawn_all(
        &self,
        mut commands: Commands,
        map_layout: Res<MapLayout>,
        mut sector_spawn_event: EventWriter<SectorWasSpawnedEvent>,
    ) {
        let mut sector_id_map = SectorIdMap::new();
        for builder in &self.data {
            let entity = builder.build(&mut commands, &map_layout, &mut sector_spawn_event);
            sector_id_map.insert(builder.coordinate, entity);
        }

        commands.insert_resource(sector_id_map);

        let asteroid_map = AsteroidIdMap::new();
        commands.insert_resource(asteroid_map);
    }
}

impl SectorSaveData {
    pub fn build(
        &self,
        commands: &mut Commands,
        map_layout: &MapLayout,
        spawn_events: &mut EventWriter<SectorWasSpawnedEvent>,
    ) -> SectorEntity {
        spawn_helpers::spawn_sector(
            commands,
            &map_layout.hex_layout,
            self.coordinate,
            self.asteroid_data.map(SectorAsteroidData::from),
            spawn_events,
        )
    }
}

impl From<SectorAsteroidSaveData> for SectorAsteroidData {
    fn from(value: SectorAsteroidSaveData) -> Self {
        SectorAsteroidData {
            average_velocity: value.average_velocity,
        }
    }
}
