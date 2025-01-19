use bevy::prelude::default;

#[derive(Default)]
pub struct InventoryElement {
    /// The amount that's currently inside this inventory, disregarding any reservations.
    pub current: u32,

    /// Reserved goods for upcoming sales from an already existing sell order.
    pub planned_selling: u32,

    /// Amount of items which are right now getting delivered to this inventory, either by production runs or ships.
    pub planned_incoming: u32,

    /// Reserved inventory space by WTB orders. Technically duplicates data inside buy orders, but is useful to have here in order to quickly calculate capacity requirements.
    pub reserved_buying: u32,

    /// Permanently reserved space for goods which are produced by this entity.
    pub reserved_production: u32,

    /// Current + Buying + Selling + Producing
    pub total: u32,
}

impl InventoryElement {
    pub fn new(amount: u32) -> Self {
        Self {
            current: amount,
            total: amount,
            ..default()
        }
    }

    /// Adds the given amount of items to this inventory.
    pub fn add(&mut self, amount: u32) {
        self.current += amount;
        self.total += amount;
    }

    /// Removes the given amount of items from this inventory.
    pub fn remove(&mut self, amount: u32) {
        self.current -= amount;
        self.total -= amount;
    }

    /// Reserves the specified amount as incoming.
    #[inline]
    pub fn add_incoming(&mut self, amount: u32) {
        self.planned_incoming += amount;
        self.total += amount;
    }

    /// Returns the highest amount of space that's reserved for this item.
    #[inline]
    pub fn reserved(&self) -> u32 {
        self.reserved_buying.max(self.reserved_production)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let element = InventoryElement::new(10);
        assert_eq!(10, element.current);
        assert_eq!(10, element.total);
        assert_eq!(0, element.planned_incoming);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.reserved_production);
    }

    #[test]
    fn add() {
        let mut element = InventoryElement::new(0);
        element.add(5);
        element.add(3);
        assert_eq!(8, element.current);
        assert_eq!(8, element.total);
        assert_eq!(0, element.planned_incoming);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.reserved_production);
    }

    #[test]
    fn remove() {
        let mut element = InventoryElement::new(10);
        element.remove(5);
        element.remove(3);
        assert_eq!(2, element.current);
        assert_eq!(2, element.total);
        assert_eq!(0, element.planned_incoming);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.reserved_production);
    }
}
