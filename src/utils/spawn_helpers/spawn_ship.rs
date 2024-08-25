use crate::components::{
    AsteroidMiningComponent, Engine, GasHarvestingComponent, Inventory, Sector, SelectableEntity,
    Ship,
};
use crate::persistence::{PersistentShipId, ShipIdMap};
use crate::session_data::ShipConfiguration;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::ship_ai::{BehaviorBuilder, TaskQueue};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{SectorEntity, ShipEntity};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Rot2, SpriteBundle};

pub fn spawn_ship(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    id: PersistentShipId,
    name: String,
    sector_query: &mut Query<&mut Sector>,
    sector: SectorEntity,
    position: Vec2,
    rotation: f32,
    velocity: ShipVelocity,
    behavior: &BehaviorBuilder,
    ship_id_map: &mut ShipIdMap,
    ship_configuration: &ShipConfiguration,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();

    let simulation_transform =
        SimulationTransform::new(sector_data.world_pos + position, Rot2::radians(rotation));

    let mut entity_commands = commands.spawn((
        Name::new(name),
        Ship::new(id, ship_configuration.id),
        SelectableEntity::Ship,
        Engine::default(),
        velocity,
        Inventory::new(ship_configuration.computed_stats.inventory_size),
        TaskQueue::new(),
        SpriteBundle {
            texture: sprites.ship.clone(),
            transform: simulation_transform.as_transform(constants::z_layers::SHIP),
            ..default()
        },
        simulation_transform,
        SimulationScale::default(),
    ));

    if let Some(asteroid_mining_amount) = ship_configuration.computed_stats.asteroid_mining_amount {
        entity_commands.insert(AsteroidMiningComponent {
            amount_per_second: asteroid_mining_amount,
        });
    }

    if let Some(gas_harvesting_amount) = ship_configuration.computed_stats.gas_harvesting_amount {
        entity_commands.insert(GasHarvestingComponent {
            amount_per_second: gas_harvesting_amount,
        });
    }

    let entity = entity_commands.id();

    ship_id_map.insert(id, ShipEntity::from(entity));
    behavior.build_and_add_default_component(commands.entity(entity));

    sector_data.add_ship(commands, sector, ShipEntity::from(entity));
}
