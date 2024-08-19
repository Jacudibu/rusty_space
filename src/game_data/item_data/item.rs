use crate::game_data::ItemId;
use crate::utils::PriceRange;
use bevy::asset::Handle;
use bevy::prelude::Image;

/// Holds all relevant data for one specific item.
pub struct ItemData {
    pub id: ItemId,
    pub name: String, // Should be determined through i18n and not stored here to allow language switching without restarts (?).
    pub icon: Handle<Image>,
    pub price: PriceRange,
}
