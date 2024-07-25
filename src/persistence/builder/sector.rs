use crate::components::SectorAsteroidData;
use crate::map_layout::MapLayout;
use crate::persistence::data::v1::{SaveDataCollection, SectorAsteroidSaveData, SectorSaveData};
use crate::persistence::{AsteroidIdMap, SectorFeatureSaveData, SectorIdMap, SectorStarSaveData};
use crate::simulation::asteroids::SectorWasSpawnedEvent;
use crate::utils::{spawn_helpers, SectorEntity};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, EventWriter, Res, Vec2};
use hexx::Hex;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    map_layout: Res<'w, MapLayout>,
    sector_spawn_event: EventWriter<'w, SectorWasSpawnedEvent>,
}

type SaveData = SaveDataCollection<SectorSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut sector_id_map = SectorIdMap::new();
    for builder in &data.data {
        let coordinate = builder.coordinate;
        let entity = builder.build(&mut args);
        sector_id_map.insert(coordinate, entity);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(sector_id_map);
    let asteroid_map = AsteroidIdMap::new();
    args.commands.insert_resource(asteroid_map);
}

impl SaveData {
    pub fn add(&mut self, hex: Hex) -> &mut SectorSaveData {
        self.data.push(SectorSaveData {
            coordinate: hex,
            features: SectorFeatureSaveData::default(),
        });
        self.data.last_mut().unwrap()
    }
}

impl SectorSaveData {
    pub fn build(&self, args: &mut Args) -> SectorEntity {
        spawn_helpers::spawn_sector(
            &mut args.commands,
            &args.map_layout.hex_layout,
            self.coordinate,
            &self.features,
            &mut args.sector_spawn_event,
        )
    }

    pub fn with_star(&mut self, data: SectorStarSaveData) -> &mut Self {
        self.features.star = Some(data);
        self
    }

    pub fn with_asteroids(&mut self, data: SectorAsteroidSaveData) -> &mut Self {
        self.features.asteroids = Some(data);
        self
    }
}
impl SectorAsteroidSaveData {
    pub fn new(average_velocity: Vec2) -> Self {
        Self {
            average_velocity,
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        }
    }
}

impl From<&SectorAsteroidSaveData> for SectorAsteroidData {
    fn from(value: &SectorAsteroidSaveData) -> Self {
        SectorAsteroidData {
            average_velocity: value.average_velocity,
        }
    }
}
