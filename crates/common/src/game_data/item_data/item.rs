use crate::game_data::ItemId;
use crate::price_range::PriceRange;
use bevy::asset::Handle;
use bevy::prelude::Image;

/// Holds all relevant data for one specific item.
pub struct ItemData {
    pub id: ItemId,

    /// User-Facing Name for this item
    /// TODO: Should be determined through i18n and not stored here to allow language switching without restarts (?).
    pub name: String,

    /// Handle to the loaded image for the icon
    pub icon: Handle<Image>,

    /// Min-Max [PriceRange] for this item.
    pub price: PriceRange,

    /// How much space a single instance of this item uses. A bigger size means less items will fit into a ship.
    pub size: u32,
}
