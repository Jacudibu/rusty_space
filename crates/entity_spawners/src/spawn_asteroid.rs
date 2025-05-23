use bevy::math::Vec2;
use bevy::prelude::{Alpha, Commands, Name, Rot2, Sprite};
use common::components::constant_velocity::ConstantVelocity;
use common::components::{Asteroid, SectorWithAsteroids, SelectableEntity};
use common::constants;
use common::game_data::{AsteroidDataId, AsteroidManifest};
use common::simulation_time::SimulationTimestamp;
use common::simulation_transform::{SimulationScale, SimulationTransform};
use common::types::entity_id_map::AsteroidIdMap;
use common::types::entity_wrappers::{AsteroidEntity, AsteroidEntityWithTimestamp, SectorEntity};
use common::types::persistent_entity_id::PersistentAsteroidId;
use leafwing_manifest::manifest::Manifest;

#[allow(clippy::too_many_arguments)]
pub fn spawn_asteroid(
    commands: &mut Commands,
    asteroid_id_map: &mut AsteroidIdMap,
    asteroid_data_id: AsteroidDataId,
    asteroid_manifest: &AsteroidManifest,
    global_pos: Vec2,
    asteroid_feature: &mut SectorWithAsteroids,
    sector_entity: SectorEntity,
    velocity: Vec2,
    ore_current: u32,
    ore_max: u32,
    sprite_rotation: f32,
    angular_velocity: f32,
    despawn_at: SimulationTimestamp,
    fading_in: bool,
) -> AsteroidEntity {
    let manifest = asteroid_manifest.get(asteroid_data_id).unwrap();
    let asteroid_id = PersistentAsteroidId::next();
    let ore_item_id = manifest.material;
    let asteroid = Asteroid::new(
        asteroid_id,
        asteroid_data_id,
        ore_item_id,
        ore_current,
        ore_max,
        despawn_at,
    );
    let scale = asteroid.scale_depending_on_current_ore_volume();

    let simulation_transform = SimulationTransform::new(global_pos, Rot2::radians(sprite_rotation));

    let entity = commands
        .spawn((
            Name::new(manifest.name.clone()),
            SelectableEntity::Asteroid(asteroid_data_id),
            ConstantVelocity::new(velocity, angular_velocity),
            Sprite {
                image: manifest.sprite.clone(),
                color: manifest
                    .sprite_color
                    .with_alpha(if fading_in { 0.0 } else { 1.0 }),
                ..Default::default()
            },
            simulation_transform.as_scaled_transform(constants::z_layers::ASTEROID, scale),
            simulation_transform,
            SimulationScale::from(scale),
            asteroid,
        ))
        .id();

    asteroid_id_map.insert(asteroid_id, AsteroidEntity::from(entity));
    asteroid_feature.add_asteroid(
        commands,
        sector_entity,
        AsteroidEntityWithTimestamp {
            entity: AsteroidEntity::from(entity),
            timestamp: despawn_at,
        },
        ore_item_id,
    );

    AsteroidEntity::from(entity)
}
