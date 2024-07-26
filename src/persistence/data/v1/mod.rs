use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

mod gate_save_data;
mod inventory_save_data;
mod planet_save_data;
mod sector_save_data;
mod ship_save_data;
mod station_save_data;
mod task_save_data;

pub use {
    gate_save_data::*, inventory_save_data::*, planet_save_data::*, sector_save_data::*,
    ship_save_data::*, station_save_data::*, task_save_data::*,
};

#[derive(Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct UniverseSaveData {
    pub gate_pairs: SaveDataCollection<GatePairSaveData>,
    pub sectors: SaveDataCollection<SectorSaveData>,
    pub ships: SaveDataCollection<ShipSaveData>,
    pub stations: SaveDataCollection<StationSaveData>,
}

#[derive(Resource, Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SaveDataCollection<T> {
    pub data: Vec<T>,
}

impl<T> Default for SaveDataCollection<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
