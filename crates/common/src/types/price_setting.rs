use crate::types::price_range::PriceRange;
use serde::{Deserialize, Serialize};

/// Defines how the price for goods is being calculated.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum PriceSetting {
    /// The price is updated dynamically depending on storage capacity, using the provided [PriceRange].
    Dynamic(PriceRange),
    /// The price is fixed to the given value.
    Fixed(u32),
}

impl PriceSetting {
    pub fn calculate_price(&self, currently_in_storage: u32, item_capacity: u32) -> u32 {
        match self {
            PriceSetting::Dynamic(range) => {
                range.calculate(currently_in_storage as f32 / item_capacity as f32)
            }
            PriceSetting::Fixed(value) => *value,
        }
    }
}
