use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, SpriteBundle, Transform};
use bevy::utils::HashMap;

use crate::components::{BuyOrders, Inventory, Sector, SelectableEntity, SellOrders, Station};
use crate::game_data::{ItemDefinition, ProductionModuleId, RecipeId, SHIPYARD_MODULE_ID};
use crate::production::{ProductionComponent, ProductionModule, ShipyardComponent, ShipyardModule};
use crate::session_data::DEBUG_SHIP_CONFIG;
use crate::utils::{SectorEntity, StationEntity};
use crate::{constants, SpriteHandles};

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
    sprites: &SpriteHandles,
    name: &str,
    pos: Vec2,
    sector_entity: SectorEntity,
    buys: Vec<&ItemDefinition>,
    sells: Vec<&ItemDefinition>,
    production: Option<MockStationProductionArgs>,
    shipyard: Option<bool>,
) {
    let mut sector = sector_query.get_mut(sector_entity.get()).unwrap();

    let pos = pos + sector.world_pos;
    let entity = commands
        .spawn((
            Name::new(name.to_string()),
            SelectableEntity::Station,
            Station,
            SpriteBundle {
                texture: sprites.station.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, constants::STATION_LAYER),
                ..default()
            },
            Inventory::new_with_content(
                constants::MOCK_INVENTORY_SIZE,
                sells
                    .iter()
                    .map(|x| (x.id, constants::MOCK_INVENTORY_SIZE))
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

    sector.add_station(commands, sector_entity, StationEntity::from(entity));
}
