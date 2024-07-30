use crate::map_layout::MapLayout;
use crate::persistence::data::v1::{SaveDataCollection, SectorAsteroidSaveData, SectorSaveData};
use crate::persistence::{
    AsteroidIdMap, PlanetIdMap, SectorFeatureSaveData, SectorIdMap, SectorPlanetSaveData,
    SectorStarSaveData,
};
use crate::simulation::asteroids::SectorWasSpawnedEvent;
use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
use crate::utils::{spawn_helpers, SectorEntity};
use crate::SpriteHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, EventWriter, Res, Vec2};
use hexx::Hex;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    map_layout: Res<'w, MapLayout>,
    orbit_directions: Res<'w, PrecomputedOrbitDirections>,
    sector_spawn_event: EventWriter<'w, SectorWasSpawnedEvent>,
}

type SaveData = SaveDataCollection<SectorSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut sector_id_map = SectorIdMap::new();
    let mut planet_id_map = PlanetIdMap::new();
    for builder in &data.data {
        let coordinate = builder.coordinate;
        let entity = builder.build(&mut args, &mut planet_id_map);
        sector_id_map.insert(coordinate, entity);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(sector_id_map);
    args.commands.insert_resource(planet_id_map);
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
    pub fn build(&self, args: &mut Args, planet_id_map: &mut PlanetIdMap) -> SectorEntity {
        spawn_helpers::spawn_sector(
            &mut args.commands,
            &args.map_layout.hex_layout,
            self.coordinate,
            &self.features,
            &mut args.sector_spawn_event,
            &args.sprites,
            planet_id_map,
            &args.orbit_directions,
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

    pub fn with_planet(&mut self, data: SectorPlanetSaveData) -> &mut Self {
        if let Some(ref mut planets) = &mut self.features.planets {
            planets.push(data);
        } else {
            self.features.planets = Some(vec![data]);
        }
        self
    }
}

impl SectorAsteroidSaveData {
    pub fn new() -> Self {
        Self {
            average_velocity: Vec2::ONE,
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        }
    }
}
