use crate::utils::PriceRange;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum PriceSetting {
    Dynamic(PriceRange),
}

impl PriceSetting {
    pub fn calculate_price(&self, storage: u32, capacity: u32) -> u32 {
        match self {
            PriceSetting::Dynamic(range) => range.calculate(storage as f32 / capacity as f32),
        }
    }
}
