use crate::components::{BuyOrders, Inventory, Sector, SelectableEntity, SellOrders, Station};
use crate::game_data::{ItemDefinition, ProductionModuleId, RecipeId, SHIPYARD_MODULE_ID};
use crate::persistence::{PersistentStationId, StationIdMap};
use crate::production::{ProductionComponent, ProductionModule, ShipyardComponent, ShipyardModule};
use crate::session_data::DEBUG_SHIP_CONFIG;
use crate::utils::{SectorEntity, StationEntity};
use crate::{constants, SpriteHandles};
use bevy::color::Color;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Sprite, SpriteBundle, Transform};
use bevy::utils::HashMap;

pub struct MockStationProductionArgs {
    modules: Vec<MockStationProductionArgElement>,
}

impl MockStationProductionArgs {
    pub fn new(modules: Vec<MockStationProductionArgElement>) -> Self {
        Self { modules }
    }

    pub fn parse(self) -> ProductionComponent {
        ProductionComponent {
            modules: HashMap::from_iter(self.modules.iter().map(|x| {
                (
                    x.module_id,
                    ProductionModule {
                        recipe: x.recipe,
                        amount: x.amount,
                        current_run_finished_at: None,
                    },
                )
            })),
        }
    }
}

pub struct MockStationProductionArgElement {
    module_id: ProductionModuleId,
    recipe: RecipeId,
    amount: u32,
}

impl MockStationProductionArgElement {
    pub fn new(module_id: ProductionModuleId, recipe: RecipeId, amount: u32) -> Self {
        Self {
            module_id,
            recipe,
            amount,
        }
    }
}

#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_station(
    commands: &mut Commands,
    sector_query: &mut Query<&mut Sector>,
    station_id_map: &mut StationIdMap,
    sprites: &SpriteHandles,
    name: &str,
    pos: Vec2,
    sector_entity: SectorEntity,
    buys: Vec<&ItemDefinition>,
    sells: Vec<&ItemDefinition>,
    production: Option<MockStationProductionArgs>,
    shipyard: Option<bool>,
) {
    let mut sector = sector_query.get_mut(sector_entity.into()).unwrap();

    let pos = pos + sector.world_pos;

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

    let _icon_entity = commands
        .spawn((
            Name::new(format!("{name} (Icon)")),
            SpriteBundle {
                texture: icon_sprite,
                transform: Transform::from_translation(pos.extend(constants::STATION_ICON_LAYER)),
                sprite: Sprite {
                    color: Color::linear_rgb(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let station_id = PersistentStationId::next();
    let entity = commands
        .spawn((
            Name::new(name.to_string()),
            SelectableEntity::Station,
            Station::new(station_id),
            SpriteBundle {
                texture: sprites.station.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, constants::STATION_LAYER),
                ..default()
            },
            Inventory::new_with_content(
                constants::MOCK_STATION_INVENTORY_SIZE,
                sells
                    .iter()
                    .map(|x| (x.id, constants::MOCK_STATION_INVENTORY_SIZE))
                    .collect(),
            ),
        ))
        .id();

    if !buys.is_empty() {
        commands.entity(entity).insert(BuyOrders::mock(buys));
    }
    if !sells.is_empty() {
        commands.entity(entity).insert(SellOrders::mock(sells));
    }

    if let Some(production) = production {
        commands.entity(entity).insert(production.parse());
    }

    if shipyard.is_some() {
        commands.entity(entity).insert(ShipyardComponent {
            modules: HashMap::from([(
                SHIPYARD_MODULE_ID,
                ShipyardModule {
                    active: Vec::new(),
                    amount: 2,
                },
            )]),
            queue: vec![DEBUG_SHIP_CONFIG; 100],
        });
    }

    station_id_map.insert(station_id, StationEntity::from(entity));
    sector.add_station(commands, sector_entity, StationEntity::from(entity));
}
