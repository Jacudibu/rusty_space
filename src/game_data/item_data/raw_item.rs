use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct RawItemData {
    pub name: String,
    pub icon: PathBuf,
    pub price_min: u32,
    pub price_max: u32,
}
