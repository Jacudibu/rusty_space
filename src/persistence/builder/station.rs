use crate::components::Sector;
use crate::game_data::{GameData, ItemDefinition, ProductionModuleId, ShipyardModuleId};
use crate::persistence::data::v1::*;
use crate::persistence::{PersistentStationId, SectorIdMap, StationIdMap};
use crate::production::{
    OngoingShipConstructionOrder, ProductionComponent, ProductionModule, ShipyardComponent,
    ShipyardModule,
};
use crate::universe_builder::LocalHexPosition;
use crate::utils::spawn_helpers;
use crate::SpriteHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};
use bevy::utils::hashbrown::HashMap;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
    game_data: Res<'w, GameData>,
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

    pub fn build(&self, args: &mut Args, station_id_map: &mut StationIdMap) {
        let sector_entity = args
            .sector_id_map
            .get_entity(&self.position.sector)
            .unwrap();

        // TODO: All custom trade data is lost right now
        let buys = self
            .buy_orders
            .clone()
            .map_or_else(Vec::new, |x| x.parse(&args.game_data));
        let sells = self
            .sell_orders
            .clone()
            .map_or_else(Vec::new, |x| x.parse(&args.game_data));

        let production = self.production_modules.clone().map(|x| x.parse());
        let shipyard = self.shipyard_modules.clone().map(|x| x.parse());

        spawn_helpers::spawn_station(
            &mut args.commands,
            &mut args.sectors,
            station_id_map,
            &args.sprites,
            &self.name,
            self.position.position,
            *sector_entity,
            buys,
            sells,
            production,
            shipyard,
        )
    }
}

impl SerializedBuyOrder {
    pub fn parse<'a>(&self, game_data: &'a GameData) -> Vec<&'a ItemDefinition> {
        self.orders
            .iter()
            .map(|x| game_data.items.get(&x.item_id).unwrap())
            .collect()
    }
}

impl SerializedSellOrder {
    pub fn parse<'a>(&self, game_data: &'a GameData) -> Vec<&'a ItemDefinition> {
        self.orders
            .iter()
            .map(|x| game_data.items.get(&x.item_id).unwrap())
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
