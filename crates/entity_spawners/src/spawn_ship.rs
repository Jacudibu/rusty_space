use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Name, Query, Rot2, Sprite, Transform};
use common::components::ship_velocity::ShipVelocity;
use common::components::task_queue::TaskQueue;
use common::components::{
    AsteroidMiner, Engine, GasHarvester, Inventory, Owner, Sector, SelectableEntity, Ship,
};
use common::constants;
use common::session_data::ShipConfiguration;
use common::simulation_transform::SimulationScale;
use common::simulation_transform::SimulationTransform;
use common::types::behavior_builder::BehaviorBuilder;
use common::types::entity_id_map::ShipIdMap;
use common::types::entity_wrappers::{SectorEntity, ShipEntity};
use common::types::persistent_entity_id::{PersistentFactionId, PersistentShipId};

/// Bundles up all components which are strictly required for spawning a ship.
/// Due to the nature of bundles, this does not include optional components such as [GasHarvester].
#[derive(Bundle)]
struct ShipSpawnDataBundle {
    name: Name,
    owner: Owner,
    ship: Ship,
    engine: Engine,
    inventory: Inventory,
    selectable_entity: SelectableEntity,
    task_queue: TaskQueue,
    velocity: ShipVelocity,
    sprite: Sprite,
    transform: Transform,
    simulation_transform: SimulationTransform,
    simulation_scale: SimulationScale,
}

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
    faction: PersistentFactionId,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();
    let simulation_transform =
        SimulationTransform::new(sector_data.world_pos + position, Rot2::radians(rotation));

    let mut entity_commands = commands.spawn(ShipSpawnDataBundle {
        name: Name::new(name),
        ship: Ship::new(id, ship_configuration.id),
        engine: Engine::from(&ship_configuration.computed_stats.engine),
        task_queue: TaskQueue::default(),
        inventory: Inventory::new(ship_configuration.computed_stats.inventory_size),
        selectable_entity: SelectableEntity::Ship(ship_configuration.id),
        velocity,
        transform: simulation_transform.as_bevy_transform(constants::z_layers::SHIP),
        simulation_transform,
        simulation_scale: SimulationScale::default(),
        sprite: Sprite::from_image(ship_configuration.sprite.clone()),
        owner: Owner {
            faction_id: faction,
        },
    });

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
