use crate::game_data::ItemId;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct RawAsteroidData {
    pub material: ItemId,
    pub amount_min: u32,
    pub amount_max: u32,
    pub sprite: PathBuf,
}
