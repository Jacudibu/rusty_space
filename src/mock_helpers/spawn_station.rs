use crate::components::{BuyOrders, Inventory, SelectableEntity, SellOrders};
use crate::game_data::{
    GameData, ItemDefinition, ProductionModuleId, RecipeId, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B,
    DEBUG_ITEM_ID_C, PRODUCTION_MODULE_A_ID, PRODUCTION_MODULE_B_ID, PRODUCTION_MODULE_C_ID,
    RECIPE_A_ID, RECIPE_B_ID, RECIPE_C_ID, SHIPYARD_MODULE_ID,
};
use crate::production::{ProductionComponent, ProductionModule, ShipyardComponent, ShipyardModule};
use crate::sectors::{DebugSectors, Sector, SectorEntity};
use crate::session_data::DEBUG_SHIP_CONFIG;
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Res, SpriteBundle, Transform};
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

    sector.add_station(commands, sector_entity, entity);
}

pub fn spawn_mock_stations(
    mut commands: Commands,
    mut sector_query: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    game_data: Res<GameData>,
    debug_sectors: Res<DebugSectors>,
) {
    spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station A",
        Vec2::new(-200.0, -200.0),
        debug_sectors.bottom_left,
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_B_ID, RECIPE_B_ID, 5),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station B",
        Vec2::new(200.0, -200.0),
        debug_sectors.center,
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_C_ID, RECIPE_C_ID, 3),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Station C",
        Vec2::new(0.0, 200.0),
        debug_sectors.center,
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_A_ID, RECIPE_A_ID, 1),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &mut sector_query,
        &sprites,
        "Shipyard",
        Vec2::new(0.0, 0.0),
        debug_sectors.center,
        vec![
            &game_data.items[&DEBUG_ITEM_ID_A],
            &game_data.items[&DEBUG_ITEM_ID_B],
            &game_data.items[&DEBUG_ITEM_ID_C],
        ],
        vec![],
        None,
        Some(true),
    );
}
