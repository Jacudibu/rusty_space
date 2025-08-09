use bevy::prelude::{Deref, DerefMut};
use common::constants;
use common::game_data::{
    ConstructableModuleId, ItemId, ProductionModuleId, RecipeId, ShipyardModuleId,
};
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::{PersistentFactionId, PersistentStationId};
use common::types::price_range::PriceRange;
use common::types::price_setting::PriceSetting;
use persistence::data::{
    ConstructionSiteSaveData, InventorySaveData, ProductionModuleQueueElementSaveData,
    ProductionModuleSaveData, ProductionSaveData, SaveDataCollection, SerializedBuyOrder,
    SerializedBuyOrderData, SerializedSellOrder, SerializedSellOrderData, ShipyardModuleSaveData,
    ShipyardSaveData, StationSaveData,
};

#[derive(Default)]
pub struct StationBuilder {
    data: Vec<IndividualStationBuilder>,
}

#[derive(Deref, DerefMut)]
pub struct IndividualStationBuilder {
    data: StationSaveData,
}

impl StationBuilder {
    pub fn add(
        &mut self,
        position: LocalHexPosition,
        name: impl Into<String>,
        owner: PersistentFactionId,
    ) -> &mut IndividualStationBuilder {
        self.data
            .push(IndividualStationBuilder::new(position, name, owner));
        self.data.last_mut().unwrap()
    }

    pub fn build(self) -> SaveDataCollection<StationSaveData> {
        SaveDataCollection {
            data: self.data.into_iter().map(|x| x.build()).collect(),
        }
    }
}

impl IndividualStationBuilder {
    pub fn new(
        position: LocalHexPosition,
        name: impl Into<String>,
        owner: PersistentFactionId,
    ) -> Self {
        Self {
            data: StationSaveData {
                id: PersistentStationId::next(),
                owner,
                position,
                name: name.into(),
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
