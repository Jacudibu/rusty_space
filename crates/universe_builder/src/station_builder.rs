use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, Deref, DerefMut, Query, Res};
use common::components::celestials::Celestial;
use common::components::production_facility::{
    ProductionFacility, ProductionModule, ProductionQueueElement, RunningProductionQueueElement,
};
use common::components::shipyard::{OngoingShipConstructionOrder, Shipyard, ShipyardModule};
use common::components::{BuyOrders, Sector, SectorWithCelestials};
use common::constants;
use common::game_data::{
    ConstructableModuleId, ItemId, ItemManifest, ProductionModuleId, RecipeId, RecipeManifest,
    ShipyardModuleId,
};
use common::types::entity_id_map::{ConstructionSiteIdMap, SectorIdMap, StationIdMap};
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::PersistentStationId;
use common::types::price_range::PriceRange;
use common::types::price_setting::PriceSetting;
use common::types::sector_position::SectorPosition;
use common::types::sprite_handles::SpriteHandles;
use entity_spawners::spawn_station::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use persistence::data::{
    ActiveShipyardOrderSaveData, ConstructionSiteSaveData, InventorySaveData,
    ProductionModuleQueueElementSaveData, ProductionModuleSaveData, ProductionSaveData,
    SaveDataCollection, SerializedBuyOrder, SerializedBuyOrderData, SerializedSellOrder,
    SerializedSellOrderData, ShipSaveData, ShipyardModuleSaveData, ShipyardSaveData,
    StationSaveData,
};

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, Option<&'static SectorWithCelestials>)>,
    sector_id_map: Res<'w, SectorIdMap>,
    items: Res<'w, ItemManifest>,
    recipes: Res<'w, RecipeManifest>,
    celestials: Query<'w, 's, &'static Celestial>,
}

#[derive(Default)]
pub struct StationBuilder {
    data: Vec<IndividualStationBuilder>,
}

#[derive(Deref, DerefMut)]
pub struct IndividualStationBuilder {
    data: StationSaveData,
}

pub fn spawn_all(data: Res<SaveDataCollection<StationSaveData>>, mut args: Args) {
    let mut station_id_map = StationIdMap::new();
    let mut construction_site_id_map = ConstructionSiteIdMap::new();
    for data in &data.data {
        build(
            data,
            &mut args,
            &mut station_id_map,
            &mut construction_site_id_map,
        );
    }

    args.commands
        .remove_resource::<SaveDataCollection<StationSaveData>>();
    args.commands.insert_resource(station_id_map);
    args.commands.insert_resource(construction_site_id_map);
}

impl StationBuilder {
    pub fn add(
        &mut self,
        position: LocalHexPosition,
        name: String,
    ) -> &mut IndividualStationBuilder {
        self.data
            .push(IndividualStationBuilder::new(position, name));
        self.data.last_mut().unwrap()
    }

    pub fn build(self) -> SaveDataCollection<StationSaveData> {
        SaveDataCollection {
            data: self.data.into_iter().map(|x| x.build()).collect(),
        }
    }
}

impl IndividualStationBuilder {
    pub fn new(position: LocalHexPosition, name: String) -> Self {
        Self {
            data: StationSaveData {
                id: PersistentStationId::next(),
                position,
                name,
                buy_orders: None,
                sell_orders: None,
                production_modules: None,
                shipyard_modules: None,
                inventory: InventorySaveData { items: Vec::new() },
                construction_site: None,
            },
        }
    }

    pub fn with_buys(&mut self, buys: Vec<ItemId>) -> &mut Self {
        if self.buy_orders.is_none() {
            self.buy_orders = Some(SerializedBuyOrder { orders: Vec::new() })
        }

        if let Some(ref mut orders) = self.buy_orders {
            orders
                .orders
                .extend(buys.into_iter().map(|x| SerializedBuyOrderData {
                    item_id: x,
                    // TODO: This requires way more data
                    amount: constants::MOCK_STATION_INVENTORY_SIZE,
                    buy_up_to: constants::MOCK_STATION_INVENTORY_SIZE,
                    price_setting: PriceSetting::Dynamic(PriceRange::new(5, 100)),
                }))
        }

        self
    }

    pub fn with_sells(&mut self, buys: Vec<ItemId>) -> &mut Self {
        if self.sell_orders.is_none() {
            self.sell_orders = Some(SerializedSellOrder { orders: Vec::new() })
        }

        if let Some(ref mut orders) = self.sell_orders {
            orders
                .orders
                .extend(buys.into_iter().map(|x| SerializedSellOrderData {
                    item_id: x,
                    // TODO: This requires way more data
                    amount: constants::MOCK_STATION_INVENTORY_SIZE,
                    keep_at_least: 0,
                    price_setting: PriceSetting::Dynamic(PriceRange::new(5, 100)),
                }))
        }

        self
    }

    pub fn with_production(
        &mut self,
        amount: u32,
        module_id: ProductionModuleId,
        recipe: RecipeId,
    ) -> &mut Self {
        if self.production_modules.is_none() {
            self.production_modules = Some(ProductionSaveData {
                modules: Vec::new(),
            });
        }

        if let Some(ref mut production) = self.production_modules {
            production.modules.push(ProductionModuleSaveData {
                amount,
                module_id,
                queued_recipes: vec![ProductionModuleQueueElementSaveData {
                    recipe,
                    is_repeating: true,
                }],
                running_recipes: Vec::new(),
            });
        }

        self
    }

    pub fn with_shipyard(&mut self, amount: u32, module_id: ShipyardModuleId) -> &mut Self {
        if self.shipyard_modules.is_none() {
            self.shipyard_modules = Some(ShipyardSaveData {
                modules: Vec::new(),
                queue: Vec::new(),
            })
        }

        if let Some(ref mut shipyard) = self.shipyard_modules {
            shipyard.modules.push(ShipyardModuleSaveData {
                amount,
                module_id,
                active: Vec::new(),
            });
        }

        self
    }

    #[deprecated(note = "Needs fixing")]
    pub fn with_construction_site(
        &mut self,
        queue: Vec<ConstructableModuleId>,
        current_progress: f32,
    ) -> &mut Self {
        return self;
        // TODO: Buy orders are currently not persisted properly
        self.construction_site = Some(ConstructionSiteSaveData {
            queue,
            current_progress,
        });
        self
    }

    pub fn build(self) -> StationSaveData {
        self.data
    }
}
fn build(
    data: &StationSaveData,
    args: &mut Args,
    station_id_map: &mut StationIdMap,
    construction_site_id_map: &mut ConstructionSiteIdMap,
) {
    let sector_entity = args
        .sector_id_map
        .get_entity(&data.position.sector)
        .unwrap();

    // TODO: All custom trade data is lost right now
    let buys = data
        .buy_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());
    let sells = data
        .sell_orders
        .clone()
        .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());

    let production = data
        .production_modules
        .clone() // TODO: Can we get rid of those clones?
        .map(parse_production_save_data);
    let shipyard = data.shipyard_modules.clone().map(parse_shipyard_save_data);

    let data = StationSpawnData {
        id: data.id,
        name: data.name.clone(),
        sector_position: SectorPosition {
            sector: *sector_entity,
            local_position: data.position.local_position,
        },
        shipyard,
        production,
        buys,
        sells,
        construction_site: data.construction_site.clone().map(|x| {
            ConstructionSiteSpawnData::new(x.queue, BuyOrders::default()) // TODO
                .with_progress(x.current_progress)
        }),
    };

    spawn_station(
        &mut args.commands,
        &mut args.sectors,
        station_id_map,
        construction_site_id_map,
        &args.sprites,
        &args.items,
        &args.recipes,
        data,
    );
}

fn parse_production_save_data(data: ProductionSaveData) -> ProductionFacility {
    ProductionFacility {
        modules: HashMap::from_iter(data.modules.iter().map(parse_production_module_save_data)),
    }
}

fn parse_production_module_save_data(
    data: &ProductionModuleSaveData,
) -> (ProductionModuleId, ProductionModule) {
    (
        data.module_id,
        ProductionModule {
            amount: data.amount,
            queued_recipes: data
                .queued_recipes
                .iter()
                .map(|x| ProductionQueueElement {
                    recipe: x.recipe,
                    is_repeating: x.is_repeating,
                })
                .collect(),
            running_recipes: data
                .running_recipes
                .iter()
                .map(|x| RunningProductionQueueElement {
                    recipe: x.recipe,
                    finished_at: x.finished_at,
                })
                .collect(),
        },
    )
}

pub fn parse_shipyard_save_data(data: ShipyardSaveData) -> Shipyard {
    Shipyard {
        modules: HashMap::from_iter(data.modules.iter().map(parse_shipyard_module_save_data)),
        queue: data.queue.clone(),
    }
}

pub fn parse_shipyard_module_save_data(
    data: &ShipyardModuleSaveData,
) -> (ShipyardModuleId, ShipyardModule) {
    (
        data.module_id,
        ShipyardModule {
            amount: data.amount,
            active: data
                .active
                .iter()
                .map(parse_active_shipyard_order)
                .collect(),
        },
    )
}

pub fn parse_active_shipyard_order(
    data: &ActiveShipyardOrderSaveData,
) -> OngoingShipConstructionOrder {
    OngoingShipConstructionOrder {
        ship_config: data.ship_config,
        finished_at: data.finished_at,
    }
}
