use crate::SpriteHandles;
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{ConstructionSiteIdMap, PersistentStationId, SectorIdMap, StationIdMap};
use crate::simulation::prelude::{ProductionQueueElement, RunningProductionQueueElement};
use crate::simulation::production::{
    OngoingShipConstructionOrder, ProductionFacility, ProductionModule, Shipyard, ShipyardModule,
};
use crate::utils::entity_spawners::{ConstructionSiteSpawnData, StationSpawnData};
use crate::utils::{PriceRange, PriceSetting, SectorPosition, entity_spawners};
use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Commands, Query, Res};
use common::components::celestials::Celestial;
use common::components::{BuyOrders, Sector, SectorWithCelestials};
use common::constants;
use common::game_data::{
    ConstructableModuleId, ItemId, ItemManifest, ProductionModuleId, RecipeId, RecipeManifest,
    ShipyardModuleId,
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

type SaveData = SaveDataCollection<StationSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut station_id_map = StationIdMap::new();
    let mut construction_site_id_map = ConstructionSiteIdMap::new();
    for builder in &data.data {
        builder.build(
            &mut args,
            &mut station_id_map,
            &mut construction_site_id_map,
        );
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(station_id_map);
    args.commands.insert_resource(construction_site_id_map);
}

impl SaveData {
    pub fn add(&mut self, position: LocalHexPosition, name: String) -> &mut StationSaveData {
        self.data.push(StationSaveData::new(position, name));
        self.data.last_mut().unwrap()
    }
}

impl StationSaveData {
    pub fn new(position: LocalHexPosition, name: String) -> Self {
        Self {
            id: PersistentStationId::next(),
            position,
            name,
            buy_orders: None,
            sell_orders: None,
            production_modules: None,
            shipyard_modules: None,
            inventory: InventorySaveData { items: Vec::new() },
            construction_site: None,
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

    pub fn build(
        &self,
        args: &mut Args,
        station_id_map: &mut StationIdMap,
        construction_site_id_map: &mut ConstructionSiteIdMap,
    ) {
        let sector_entity = args
            .sector_id_map
            .get_entity(&self.position.sector)
            .unwrap();

        // TODO: All custom trade data is lost right now
        let buys = self
            .buy_orders
            .clone()
            .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());
        let sells = self
            .sell_orders
            .clone()
            .map_or_else(Vec::new, |x| x.orders.iter().map(|x| x.item_id).collect());

        let production = self.production_modules.clone().map(|x| x.parse());
        let shipyard = self.shipyard_modules.clone().map(|x| x.parse());

        let data = StationSpawnData {
            id: self.id,
            name: self.name.clone(),
            sector_position: SectorPosition {
                sector: *sector_entity,
                local_position: self.position.position,
            },
            shipyard,
            production,
            buys,
            sells,
            construction_site: self.construction_site.clone().map(|x| {
                ConstructionSiteSpawnData::new(x.queue, BuyOrders::default()) // TODO
                    .with_progress(x.current_progress)
            }),
        };

        entity_spawners::spawn_station(
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
}

impl ProductionSaveData {
    pub fn parse(&self) -> ProductionFacility {
        ProductionFacility {
            modules: HashMap::from_iter(self.modules.iter().map(|x| x.parse())),
        }
    }
}

impl ProductionModuleSaveData {
    pub fn parse(&self) -> (ProductionModuleId, ProductionModule) {
        (
            self.module_id,
            ProductionModule {
                amount: self.amount,
                queued_recipes: self
                    .queued_recipes
                    .iter()
                    .map(|x| ProductionQueueElement {
                        recipe: x.recipe,
                        is_repeating: x.is_repeating,
                    })
                    .collect(),
                running_recipes: self
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
}

impl ShipyardSaveData {
    pub fn parse(&self) -> Shipyard {
        Shipyard {
            modules: HashMap::from_iter(self.modules.iter().map(|x| x.parse())),
            queue: self.queue.clone(),
        }
    }
}

impl ShipyardModuleSaveData {
    pub fn parse(&self) -> (ShipyardModuleId, ShipyardModule) {
        (
            self.module_id,
            ShipyardModule {
                amount: self.amount,
                active: self.active.iter().map(|x| x.parse()).collect(),
            },
        )
    }
}

impl ActiveShipyardOrderSaveData {
    pub fn parse(&self) -> OngoingShipConstructionOrder {
        OngoingShipConstructionOrder {
            ship_config: self.ship_config,
            finished_at: self.finished_at,
        }
    }
}
