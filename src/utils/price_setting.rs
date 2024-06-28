use crate::utils::PriceRange;

pub enum PriceSetting {
    Fixed(u32),
    Dynamic(PriceRange),
}

impl PriceSetting {
    pub fn calculate_price(&self, storage: u32, capacity: u32) -> u32 {
        match self {
            PriceSetting::Fixed(value) => *value,
            PriceSetting::Dynamic(range) => range.calculate(storage as f32 / capacity as f32),
        }
    }
}
