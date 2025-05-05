use bevy::math::Vec2;
use bevy::prelude::{Commands, Name, Query, Rot2, Sprite};
use common::components::ship_velocity::ShipVelocity;
use common::components::task_queue::TaskQueue;
use common::components::{
    AsteroidMiner, Engine, GasHarvester, Inventory, Sector, SelectableEntity, Ship,
};
use common::constants;
use common::session_data::ShipConfiguration;
use common::simulation_transform::SimulationScale;
use common::simulation_transform::SimulationTransform;
use common::types::behavior_builder::BehaviorBuilder;
use common::types::entity_id_map::ShipIdMap;
use common::types::entity_wrappers::{SectorEntity, ShipEntity};
use common::types::persistent_entity_id::PersistentShipId;

pub fn spawn_ship(
    commands: &mut Commands,
    id: PersistentShipId,
    name: String,
    sector_query: &mut Query<&mut Sector>,
    sector: SectorEntity,
    position: Vec2,
    rotation: f32,
    velocity: ShipVelocity,
    behavior: BehaviorBuilder,
    ship_id_map: &mut ShipIdMap,
    ship_configuration: &ShipConfiguration,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();

    let simulation_transform =
        SimulationTransform::new(sector_data.world_pos + position, Rot2::radians(rotation));

    let mut entity_commands = commands.spawn((
        Name::new(name),
        Ship::new(id, ship_configuration.id),
        SelectableEntity::Ship(ship_configuration.id),
        Engine::from(&ship_configuration.computed_stats.engine),
        velocity,
        Inventory::new(ship_configuration.computed_stats.inventory_size),
        TaskQueue::default(),
        Sprite::from_image(ship_configuration.sprite.clone()),
        simulation_transform.as_bevy_transform(constants::z_layers::SHIP),
        simulation_transform,
        SimulationScale::default(),
    ));

    if let Some(asteroid_mining_amount) = ship_configuration.computed_stats.asteroid_mining_amount {
        entity_commands.insert(AsteroidMiner {
            amount_per_second: asteroid_mining_amount,
        });
    }

    if let Some(gas_harvesting_amount) = ship_configuration.computed_stats.gas_harvesting_amount {
        entity_commands.insert(GasHarvester {
            amount_per_second: gas_harvesting_amount,
        });
    }

    let entity = entity_commands.id();

    ship_id_map.insert(id, ShipEntity::from(entity));
    behavior.build_and_add_default_component(commands.entity(entity));

    sector_data.add_ship(commands, sector, ShipEntity::from(entity));
}
