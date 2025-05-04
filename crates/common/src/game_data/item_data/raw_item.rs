use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct RawItemData {
    /// User-Facing name of the item
    pub name: String,

    /// Path to the icon file for this item
    pub icon: PathBuf,

    /// Minimum suggested price value
    pub price_min: u32,

    /// Maximum suggested price value
    pub price_max: u32,

    /// The size of the item
    pub size: u32,
}
