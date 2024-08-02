use crate::components::{
    BuyOrders, InteractionQueue, Inventory, Sector, SelectableEntity, SellOrders, Station,
};
use crate::game_data::ItemDefinition;
use crate::persistence::{PersistentStationId, StationIdMap};
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::production::{ProductionComponent, ShipyardComponent};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{SectorEntity, StationEntity};
use crate::{constants, SpriteHandles};
use bevy::color::Color;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Sprite, SpriteBundle};

#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_station(
    commands: &mut Commands,
    sector_query: &mut Query<&mut Sector>,
    station_id_map: &mut StationIdMap,
    sprites: &SpriteHandles,
    id: PersistentStationId,
    name: &str,
    local_pos: Vec2,
    sector_entity: SectorEntity,
    buys: Vec<&ItemDefinition>,
    sells: Vec<&ItemDefinition>,
    production: Option<ProductionComponent>,
    shipyard: Option<ShipyardComponent>,
) {
    let mut sector = sector_query.get_mut(sector_entity.into()).unwrap();

    let icon_sprite = match sells.first() {
        None => {
            if shipyard.is_some() {
                sprites.icon_ship.clone()
            } else {
                sprites.icon_unknown.clone()
            }
        }
        Some(item) => match item.id {
            // somehow matching with aliased-constants doesn't work?
            1 => sprites.icon_item_a.clone(),
            2 => sprites.icon_item_b.clone(),
            3 => sprites.icon_item_c.clone(),
            _ => sprites.icon_unknown.clone(),
        },
    };

    let simulation_transform = SimulationTransform::from_translation(local_pos + sector.world_pos);

    let _icon_entity = commands
        .spawn((
            Name::new(format!("{name} (Icon)")),
            SpriteBundle {
                texture: icon_sprite,
                transform: simulation_transform.as_transform(constants::z_layers::STATION_ICON),
                sprite: Sprite {
                    color: Color::linear_rgb(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let entity = commands
        .spawn((
            Name::new(name.to_string()),
            SelectableEntity::Station,
            Station::new(id),
            SpriteBundle {
                texture: sprites.station.clone(),
                transform: simulation_transform.as_transform(constants::z_layers::STATION),
                ..default()
            },
            Inventory::new_with_content(
                constants::MOCK_STATION_INVENTORY_SIZE,
                sells
                    .iter()
                    .map(|x| (x.id, constants::MOCK_STATION_INVENTORY_SIZE))
                    .collect(),
            ),
            InteractionQueue::new(constants::SIMULTANEOUS_STATION_INTERACTIONS),
            simulation_transform,
            SimulationScale::default(),
        ))
        .id();

    if !buys.is_empty() {
        commands.entity(entity).insert(BuyOrders::mock(buys));
    }
    if !sells.is_empty() {
        commands.entity(entity).insert(SellOrders::mock(sells));
    }

    if let Some(production) = production {
        commands.entity(entity).insert(production);
    }

    if let Some(shipyard) = shipyard {
        commands.entity(entity).insert(shipyard);
    }

    station_id_map.insert(id, StationEntity::from(entity));
    sector.add_station(commands, sector_entity, StationEntity::from(entity));
}
