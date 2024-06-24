use crate::data::ItemId;
use crate::utils::TradeIntent;
use bevy::log::error;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct Inventory {
    pub capacity: u32,
    inventory: HashMap<ItemId, InventoryElement>,
}

#[derive(Default)]
pub struct InventoryElement {
    pub currently_available: u32,
    pub planned_buying: u32,
    pub planned_selling: u32,
    pub total: u32,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            inventory: HashMap::new(),
        }
    }

    pub fn new_with_content(capacity: u32, content: Vec<(ItemId, u32)>) -> Self {
        let mut result = Self::new(capacity);

        for (item_id, amount) in content {
            result.inventory.insert(
                item_id,
                InventoryElement {
                    currently_available: amount,
                    ..Default::default()
                },
            );
        }

        result
    }

    pub fn inventory(&self) -> &HashMap<ItemId, InventoryElement> {
        &self.inventory
    }

    pub fn used(&self) -> u32 {
        self.inventory
            .iter()
            .fold(0, |acc, (_, value)| acc + value.currently_available)
    }

    pub fn ratio(&self) -> f32 {
        self.used() as f32 / self.capacity as f32
    }

    pub fn get(&self, item_id: &ItemId) -> Option<&InventoryElement> {
        self.inventory.get(item_id)
    }

    pub fn create_order(&mut self, item_id: ItemId, intent: TradeIntent, amount: u32) {
        if let Some(inventory) = self.inventory.get_mut(&item_id) {
            match intent {
                TradeIntent::Buy => {
                    inventory.planned_buying += amount;
                    inventory.total += amount;
                }
                TradeIntent::Sell => {
                    inventory.planned_selling += amount;
                    inventory.total -= amount;
                }
            }
        } else {
            match intent {
                TradeIntent::Buy => {
                    let item = InventoryElement {
                        total: amount,
                        currently_available: 0,
                        planned_buying: amount,
                        planned_selling: 0,
                    };
                    self.inventory.insert(item_id, item);
                }
                TradeIntent::Sell => {
                    error!("How are we supposed to sell something if the item isn't even tracked inside our inventory yet?")
                }
            }
        }
    }

    pub fn complete_order(&mut self, item_id: ItemId, intent: TradeIntent, amount: u32) {
        let Some(inventory) = self.inventory.get_mut(&item_id) else {
            error!("Inventory Entry did not exist on order completion!");
            return;
        };

        match intent {
            TradeIntent::Buy => {
                inventory.currently_available += amount;
                inventory.planned_buying -= amount;
            }
            TradeIntent::Sell => {
                inventory.currently_available -= amount;
                inventory.planned_selling -= amount;
            }
        }
    }
}
