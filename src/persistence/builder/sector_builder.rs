use crate::game_data::{AsteroidDataId, AsteroidManifest};
use crate::map_layout::MapLayout;
use crate::persistence::data::v1::{SaveDataCollection, SectorAsteroidSaveData, SectorSaveData};
use crate::persistence::{
    AsteroidIdMap, AsteroidSaveData, PersistentAsteroidId, PlanetIdMap, SectorFeatureSaveData,
    SectorIdMap, SectorPlanetSaveData, SectorStarSaveData,
};
use crate::simulation::time::SimulationTimestamp;
use crate::utils::{SectorEntity, UniverseSeed, entity_spawners};
use crate::{SpriteHandles, constants};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Circle, Commands, Res, ShapeSample, Vec2};
use hexx::Hex;
use leafwing_manifest::manifest::Manifest;
use rand::Rng;
use rand::distributions::Distribution;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    map_layout: Res<'w, MapLayout>,
    asteroid_manifest: Res<'w, AsteroidManifest>,
}

type SaveData = SaveDataCollection<SectorSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut sector_id_map = SectorIdMap::new();
    let mut asteroid_id_map = AsteroidIdMap::new();
    let mut planet_id_map = PlanetIdMap::new();
    for builder in &data.data {
        let coordinate = builder.coordinate;
        let entity = builder.build(&mut args, &mut asteroid_id_map, &mut planet_id_map);
        sector_id_map.insert(coordinate, entity);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(sector_id_map);
    args.commands.insert_resource(asteroid_id_map);
    args.commands.insert_resource(planet_id_map);
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
    pub fn build(
        &self,
        args: &mut Args,
        asteroid_id_map: &mut AsteroidIdMap,
        planet_id_map: &mut PlanetIdMap,
    ) -> SectorEntity {
        entity_spawners::spawn_sector(
            &mut args.commands,
            &args.map_layout.hex_layout,
            self.coordinate,
            &self.features,
            &args.sprites,
            asteroid_id_map,
            planet_id_map,
            &args.asteroid_manifest,
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
        if let Some(planets) = &mut self.features.planets {
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
            asteroid_materials: Vec::new(),
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        }
    }

    pub fn with_average_velocity(mut self, velocity: Vec2) -> Self {
        self.average_velocity = velocity;
        self
    }

    pub fn add_random_live_asteroids(
        mut self,
        sector_hex: Hex,
        amount: usize,
        universe_seed: &UniverseSeed,
        map_layout: &MapLayout,
        asteroid_manifest: &AsteroidManifest,
        asteroid_data_id: AsteroidDataId,
    ) -> Self {
        let shape = Circle::new(constants::SECTOR_SIZE * 0.8);
        let position_rng = universe_seed.for_sector(sector_hex, "positions");
        let mut inner_rng = universe_seed.for_sector(sector_hex, "everything_else");

        let sector_pos = map_layout.hex_layout.hex_to_world_pos(sector_hex);

        let manifest = asteroid_manifest.get(asteroid_data_id).unwrap();
        if !self.asteroid_materials.contains(&manifest.material) {
            self.asteroid_materials.push(manifest.material);
        }

        self.live_asteroids.extend(shape.interior_dist().sample_iter(position_rng).take(amount).map(|local_position| {
                let velocity = Vec2::new(
                    self.average_velocity.x
                        * inner_rng.gen_range(constants::ASTEROID_VELOCITY_RANDOM_RANGE),
                    self.average_velocity.y
                        * inner_rng.gen_range(constants::ASTEROID_VELOCITY_RANDOM_RANGE),
                );

                let despawn_after = crate::simulation::asteroids::helpers::calculate_milliseconds_until_asteroid_leaves_hexagon(
                    map_layout.hex_edge_vertices,
                    local_position,
                    velocity,
                );

                let rotation = inner_rng.gen_range(constants::ASTEROID_ROTATION_RANDOM_RANGE);
                let ore = inner_rng.gen_range(constants::ASTEROID_ORE_RANGE);
                AsteroidSaveData {
                    id: PersistentAsteroidId::next(),
                    manifest_id: asteroid_data_id,
                    position: local_position + sector_pos,
                    velocity,
                    rotation_degrees: rotation * std::f32::consts::PI * 1000.0,
                    angular_velocity: rotation,
                    ore_item_id: manifest.material,
                    ore_current: ore,
                    ore_max: ore,
                    lifetime: SimulationTimestamp::from(despawn_after),
                }
            }));

        self
    }
}
