use crate::utils::PriceRange;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum PriceSetting {
    Dynamic(PriceRange),
}

impl PriceSetting {
    pub fn calculate_price(&self, currently_in_storage: u32, item_capacity: u32) -> u32 {
        match self {
            PriceSetting::Dynamic(range) => {
                range.calculate(currently_in_storage as f32 / item_capacity as f32)
            }
        }
    }
}
