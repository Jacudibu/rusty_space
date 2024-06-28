use crate::components::{BuyOrders, Inventory, SelectableEntity, SellOrders};
use crate::game_data::{
    GameData, ItemDefinition, ProductionModuleId, RecipeId, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B,
    DEBUG_ITEM_ID_C, PRODUCTION_MODULE_A_ID, PRODUCTION_MODULE_B_ID, PRODUCTION_MODULE_C_ID,
    RECIPE_A_ID, RECIPE_B_ID, RECIPE_C_ID, SHIPYARD_MODULE_ID,
};
use crate::production::{ProductionComponent, ProductionModule, ShipyardComponent, ShipyardModule};
use crate::session_data::DEBUG_SHIP_CONFIG;
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Res, SpriteBundle, Transform};
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

pub fn spawn_station(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: &str,
    pos: Vec2,
    buys: Vec<&ItemDefinition>,
    sells: Vec<&ItemDefinition>,
    production: Option<MockStationProductionArgs>,
    shipyard: Option<bool>,
) {
    let station = commands
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
        commands.entity(station).insert(BuyOrders::mock(buys));
    }
    if !sells.is_empty() {
        commands.entity(station).insert(SellOrders::mock(sells));
    }

    if let Some(production) = production {
        commands.entity(station).insert(production.parse());
    }

    if shipyard.is_some() {
        commands.entity(station).insert(ShipyardComponent {
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
}

pub fn spawn_mock_stations(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    game_data: Res<GameData>,
) {
    spawn_station(
        &mut commands,
        &sprites,
        "Station A",
        Vec2::new(-200.0, -200.0),
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_B_ID, RECIPE_B_ID, 5),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station B",
        Vec2::new(200.0, -200.0),
        vec![&game_data.items[&DEBUG_ITEM_ID_B]],
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_C_ID, RECIPE_C_ID, 3),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station C",
        Vec2::new(0.0, 200.0),
        vec![&game_data.items[&DEBUG_ITEM_ID_C]],
        vec![&game_data.items[&DEBUG_ITEM_ID_A]],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_A_ID, RECIPE_A_ID, 1),
        ])),
        None,
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Shipyard",
        Vec2::new(0.0, 0.0),
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
