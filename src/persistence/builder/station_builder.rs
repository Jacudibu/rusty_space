use crate::components::Sector;
use crate::game_data::{
    ItemData, ItemId, ItemManifest, ProductionModuleId, RecipeId, RecipeManifest, ShipyardModuleId,
};
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{PersistentStationId, SectorIdMap, StationIdMap};
use crate::simulation::production::{
    OngoingShipConstructionOrder, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::utils::{entity_spawners, PriceRange, PriceSetting};
use crate::{constants, SpriteHandles};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};
use bevy::utils::hashbrown::HashMap;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
    items: Res<'w, ItemManifest>,
    recipes: Res<'w, RecipeManifest>,
}

type SaveData = SaveDataCollection<StationSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut station_id_map = StationIdMap::new();
    for builder in &data.data {
        builder.build(&mut args, &mut station_id_map);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(station_id_map);
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
                recipe,
                finished_at: None,
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

    pub fn build(&self, args: &mut Args, station_id_map: &mut StationIdMap) {
        let sector_entity = args
            .sector_id_map
            .get_entity(&self.position.sector)
            .unwrap();

        // TODO: All custom trade data is lost right now
        let buys = self
            .buy_orders
            .clone()
            .map_or_else(Vec::new, |x| x.parse(&args.items));
        let sells = self
            .sell_orders
            .clone()
            .map_or_else(Vec::new, |x| x.parse(&args.items));

        let production = self.production_modules.clone().map(|x| x.parse());
        let shipyard = self.shipyard_modules.clone().map(|x| x.parse());

        entity_spawners::spawn_station(
            &mut args.commands,
            &mut args.sectors,
            station_id_map,
            &args.sprites,
            self.id,
            &self.name,
            self.position.position,
            *sector_entity,
            buys,
            sells,
            production,
            shipyard,
            &args.items,
            &args.recipes,
        )
    }
}

impl SerializedBuyOrder {
    pub fn parse<'a>(&self, items: &'a ItemManifest) -> Vec<&'a ItemData> {
        self.orders
            .iter()
            .map(|x| items.get_by_ref(&x.item_id).unwrap())
            .collect()
    }
}

impl SerializedSellOrder {
    pub fn parse<'a>(&self, items: &'a ItemManifest) -> Vec<&'a ItemData> {
        self.orders
            .iter()
            .map(|x| items.get_by_ref(&x.item_id).unwrap())
            .collect()
    }
}

impl ProductionSaveData {
    pub fn parse(&self) -> ProductionComponent {
        ProductionComponent {
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
                recipe: self.recipe,
                current_run_finished_at: self.finished_at,
            },
        )
    }
}

impl ShipyardSaveData {
    pub fn parse(&self) -> ShipyardComponent {
        ShipyardComponent {
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
