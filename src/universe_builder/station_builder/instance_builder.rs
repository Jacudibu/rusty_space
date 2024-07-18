use crate::components::Sector;
use crate::game_data::{GameData, ItemId, ProductionModuleId, RecipeId, ShipyardModuleId};
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{SectorIdMap, StationIdMap};
use crate::session_data::ShipConfigId;
use crate::utils::spawn_helpers::{
    spawn_station, MockStationProductionArgElement, MockStationProductionArgs,
};
use crate::utils::SimulationTimestamp;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query};

pub struct StationSpawnDataInstanceBuilder {
    pub position: LocalHexPosition,
    pub name: String,
    pub buys: Vec<ItemId>,
    pub sells: Vec<ItemId>,
    pub production_modules: Vec<StationSpawnDataProductionElement>,
    pub shipyard_modules: Vec<StationSpawnDataShipyardElement>,
}

impl StationSpawnDataInstanceBuilder {
    pub fn new(position: LocalHexPosition, name: String) -> StationSpawnDataInstanceBuilder {
        Self {
            position,
            name,
            production_modules: Vec::new(),
            shipyard_modules: Vec::new(),
            buys: Vec::new(),
            sells: Vec::new(),
        }
    }

    pub fn with_buys(&mut self, mut buys: Vec<ItemId>) -> &mut Self {
        self.buys.append(&mut buys);
        self
    }

    pub fn with_sells(&mut self, mut sells: Vec<ItemId>) -> &mut Self {
        self.sells.append(&mut sells);
        self
    }

    pub fn with_production(
        &mut self,
        amount: u32,
        module_id: ProductionModuleId,
        active_recipe: RecipeId,
    ) -> &mut Self {
        self.production_modules
            .push(StationSpawnDataProductionElement {
                amount,
                module_id,
                active_recipe,
                finishes_at: None,
            });
        self
    }

    pub fn with_shipyard(&mut self, amount: u32, module_id: ShipyardModuleId) -> &mut Self {
        self.shipyard_modules.push(StationSpawnDataShipyardElement {
            amount,
            module_id,
            queue: Vec::new(),
        });
        self
    }

    pub fn build(
        &self,
        commands: &mut Commands,
        sectors: &mut Query<&mut Sector>,
        station_id_map: &mut StationIdMap,
        sprites: &SpriteHandles,
        sector_id_map: &SectorIdMap,
        game_data: &GameData,
    ) {
        // let sector_entity = sector_id_map.get_entity(&self.position.sector).unwrap();
        //
        // // TODO: Dynamically create those depending on production
        // //         Manual overrides will come later, and shouldn't be a priority here for now
        // let buys = self.buys.iter().map(|x| &game_data.items[x]).collect();
        // let sells = self.sells.iter().map(|x| &game_data.items[x]).collect();
        //
        // let production = if self.production_modules.is_empty() {
        //     None
        // } else {
        //     // TODO: Skip MockProductionArgs once test_universe has been replaced
        //     Some(MockStationProductionArgs::new(
        //         self.production_modules
        //             .iter()
        //             .map(|x| {
        //                 MockStationProductionArgElement::new(x.module_id, x.active_recipe, x.amount)
        //             })
        //             .collect(),
        //     ))
        // };
        // let shipyard = if self.shipyard_modules.is_empty() {
        //     None
        // } else {
        //     // TODO: ??? This should definitely be more than a boolean now. Fix once test_universe has been replaced
        //     Some(true)
        // };

        // spawn_station(
        //     commands,
        //     sectors,
        //     station_id_map,
        //     sprites,
        //     &self.name,
        //     self.position.position,
        //     *sector_entity,
        //     buys,
        //     sells,
        //     production,
        //     shipyard,
        // )
    }
}

pub struct StationSpawnDataProductionElement {
    pub amount: u32,
    pub module_id: ProductionModuleId,
    pub active_recipe: RecipeId,
    pub finishes_at: Option<SimulationTimestamp>,
}

pub struct StationSpawnDataShipyardElement {
    pub amount: u32,
    pub module_id: ShipyardModuleId,
    pub queue: Vec<ShipConfigId>,
}
