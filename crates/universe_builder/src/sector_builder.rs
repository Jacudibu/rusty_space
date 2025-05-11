use crate::celestial_builder::SectorCelestialBuilder;
use bevy::prelude::{Circle, Deref, DerefMut, ShapeSample, Vec2};
use common::constants;
use common::game_data::{AsteroidDataId, AsteroidManifest};
use common::shared_logic::calculate_milliseconds_until_asteroid_leaves_hexagon;
use common::simulation_time::SimulationTimestamp;
use common::types::map_layout::MapLayout;
use common::types::persistent_entity_id::PersistentAsteroidId;
use common::types::universe_seed::UniverseSeed;
use hexx::Hex;
use leafwing_manifest::manifest::Manifest;
use persistence::data::{
    AsteroidSaveData, SaveDataCollection, SectorAsteroidSaveData, SectorCelestialsSaveData,
    SectorFeatureSaveData, SectorSaveData,
};
use rand::Rng;
use rand::distributions::Distribution;

#[derive(Deref, DerefMut, Default)]
pub struct SectorBuilder {
    data: Vec<IndividualSectorBuilder>,
}

#[derive(Deref, DerefMut)]
pub struct IndividualSectorBuilder {
    data: SectorSaveData,
}
#[derive(Deref, DerefMut)]
pub struct SectorAsteroidBuilder {
    data: SectorAsteroidSaveData,
}

impl SectorBuilder {
    pub fn add(&mut self, hex: Hex) -> &mut IndividualSectorBuilder {
        self.data.push(IndividualSectorBuilder {
            data: SectorSaveData {
                coordinate: hex,
                features: SectorFeatureSaveData::default(),
            },
        });
        self.data.last_mut().unwrap()
    }

    pub fn build(self) -> SaveDataCollection<SectorSaveData> {
        SaveDataCollection {
            data: self.data.into_iter().map(|x| x.data).collect(),
        }
    }
}

impl IndividualSectorBuilder {
    pub fn with_celestial(&mut self, builder: SectorCelestialBuilder) -> &mut Self {
        if let Some(data) = &mut self.features.celestials {
            data.celestials.push(builder.data);
        } else {
            self.features.celestials = Some(SectorCelestialsSaveData {
                center_mass: builder.data.mass,
                celestials: vec![builder.data],
            });
        }
        self
    }

    pub fn with_asteroids(&mut self, asteroid_builder: SectorAsteroidBuilder) -> &mut Self {
        self.features.asteroids = Some(asteroid_builder.data);
        self
    }
}

impl SectorAsteroidBuilder {
    pub fn new() -> Self {
        Self {
            data: SectorAsteroidSaveData {
                average_velocity: Vec2::ONE,
                asteroid_materials: Vec::new(),
                live_asteroids: Vec::new(),
                respawning_asteroids: Vec::new(),
            },
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

        let velocity = self.average_velocity;

        self.live_asteroids.extend(
            shape
                .interior_dist()
                .sample_iter(position_rng)
                .take(amount)
                .map(|local_position| {
                    let velocity = Vec2::new(
                        velocity.x * inner_rng.gen_range(constants::ASTEROID_VELOCITY_RANDOM_RANGE),
                        velocity.y * inner_rng.gen_range(constants::ASTEROID_VELOCITY_RANDOM_RANGE),
                    );

                    let despawn_after = calculate_milliseconds_until_asteroid_leaves_hexagon(
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
                }),
        );

        self
    }
}
