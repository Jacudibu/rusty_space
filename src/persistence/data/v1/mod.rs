use serde::{Deserialize, Serialize};

mod gate_save_data;
mod inventory_save_data;
mod sector_save_data;
mod ship_save_data;
mod station_save_data;
mod task_save_data;

pub use {
    gate_save_data::*, inventory_save_data::*, sector_save_data::*, ship_save_data::*,
    station_save_data::*, task_save_data::*,
};

#[derive(Serialize, Deserialize)]
pub struct UniverseSaveData {
    pub gate_pairs: Vec<GatePairSaveData>,
    pub sectors: Vec<SectorSaveData>,
    pub ships: Vec<ShipSaveData>,
    pub stations: Vec<StationSaveData>,
}
