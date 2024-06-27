use crate::components::{BuyOrders, Inventory, SelectableEntity, SellOrders};
use crate::data::{
    GameData, ItemDefinition, ProductionModuleId, RecipeId, DEBUG_ITEM_ID_A, DEBUG_ITEM_ID_B,
    DEBUG_ITEM_ID_C, PRODUCTION_MODULE_A_ID, PRODUCTION_MODULE_B_ID, PRODUCTION_MODULE_C_ID,
    RECIPE_A_ID, RECIPE_B_ID, RECIPE_C_ID,
};
use crate::production::{ProductionComponent, ProductionModule};
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
    buys: &ItemDefinition,
    sells: &ItemDefinition,
    production: Option<MockStationProductionArgs>,
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
                vec![(sells.id, constants::MOCK_INVENTORY_SIZE)],
            ),
            BuyOrders::mock_buying_item(buys),
            SellOrders::mock_selling_item(sells),
        ))
        .id();

    if let Some(production) = production {
        commands.entity(station).insert(production.parse());
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
        &game_data.items[&DEBUG_ITEM_ID_A],
        &game_data.items[&DEBUG_ITEM_ID_B],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_B_ID, RECIPE_B_ID, 5),
        ])),
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station B",
        Vec2::new(200.0, -200.0),
        &game_data.items[&DEBUG_ITEM_ID_B],
        &game_data.items[&DEBUG_ITEM_ID_C],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_C_ID, RECIPE_C_ID, 3),
        ])),
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station C",
        Vec2::new(0.0, 200.0),
        &game_data.items[&DEBUG_ITEM_ID_C],
        &game_data.items[&DEBUG_ITEM_ID_A],
        Some(MockStationProductionArgs::new(vec![
            MockStationProductionArgElement::new(PRODUCTION_MODULE_A_ID, RECIPE_A_ID, 1),
        ])),
    );
}
