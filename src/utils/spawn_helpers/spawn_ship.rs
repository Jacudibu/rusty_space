use crate::components::{Engine, Inventory, Sector, SelectableEntity, Ship};
use crate::persistence::{PersistentShipId, ShipIdMap};
use crate::simulation::physics::ShipVelocity;
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
    behavior: &BehaviorBuilder,
    ship_id_map: &mut ShipIdMap,
) {
    let mut sector_data = sector_query.get_mut(sector.into()).unwrap();

    let simulation_transform = SimulationTransform::new(
        sector_data.world_pos + position,
        Rot2::radians(rotation),
        1.0,
    );

    let entity = commands
        .spawn((
            Name::new(name),
            Ship::new(id),
            SelectableEntity::Ship,
            Engine::default(),
            ShipVelocity::default(),
            Inventory::new(100),
            TaskQueue::new(),
            SpriteBundle {
                texture: sprites.ship.clone(),
                transform: simulation_transform.as_transform(constants::z_layers::SHIP),
                ..default()
            },
            simulation_transform,
        ))
        .id();

    ship_id_map.insert(id, ShipEntity::from(entity));
    behavior.build_and_add_default_component(commands.entity(entity));

    sector_data.add_ship(commands, sector, ShipEntity::from(entity));
}
