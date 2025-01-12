use bevy::prelude::default;

#[derive(Default)]
pub struct InventoryElement {
    /// The amount that's currently inside this inventory, disregarding any reservations.
    pub current: u32,

    /// Reserved space for upcoming purchases from an already existing buy order.
    pub planned_buying: u32,

    /// Reserved goods for upcoming sales from an already existing sell order.
    pub planned_selling: u32,

    /// Reserved space for goods that will leave a production line soon.
    pub planned_producing: u32,

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

    pub fn add(&mut self, amount: u32) {
        self.current += amount;
        self.total += amount;
    }

    pub fn remove(&mut self, amount: u32) {
        self.current -= amount;
        self.total -= amount;
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
        assert_eq!(0, element.planned_buying);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.planned_producing);
    }

    #[test]
    fn add() {
        let mut element = InventoryElement::new(0);
        element.add(5);
        element.add(3);
        assert_eq!(8, element.current);
        assert_eq!(8, element.total);
        assert_eq!(0, element.planned_buying);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.planned_producing);
    }

    #[test]
    fn remove() {
        let mut element = InventoryElement::new(10);
        element.remove(5);
        element.remove(3);
        assert_eq!(2, element.current);
        assert_eq!(2, element.total);
        assert_eq!(0, element.planned_buying);
        assert_eq!(0, element.planned_selling);
        assert_eq!(0, element.planned_producing);
    }
}
