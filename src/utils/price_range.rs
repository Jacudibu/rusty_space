use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PriceRange {
    pub min: u32,
    pub max: u32,
}

impl PriceRange {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }

    /// Linearly interpolates the price given the provided storage capacity percentage.
    ///
    /// # Returns
    ///
    /// self.min for percentage == 1.0 (storage is full -> minimum price)
    ///
    /// self.max for percentage == 0.0 (storage is empty -> maximum price )
    pub fn calculate(&self, percentage: f32) -> u32 {
        let result = percentage * self.min as f32 + (1.0 - percentage) * self.max as f32;
        result.round() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_returns_correct_percentiles() {
        let range = PriceRange::new(0, 100);

        assert_eq!(range.calculate(0.0), 100);
        assert_eq!(range.calculate(0.5), 50);
        assert_eq!(range.calculate(1.0), 0);
    }
}
