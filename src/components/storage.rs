use crate::ids::ItemId;
use bevy::log::error;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct Storage {
    pub capacity: u32,
    inventory: HashMap<ItemId, u32>,
}

impl Storage {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            inventory: HashMap::new(),
        }
    }

    pub fn used(&self) -> u32 {
        self.inventory.iter().fold(0, |acc, (_, value)| acc + value)
    }

    pub fn add_item(&mut self, item_id: ItemId, amount: u32) {
        if self.inventory.contains_key(&item_id) {
            *self.inventory.get_mut(&item_id).unwrap() += amount;
        } else {
            self.inventory.insert(item_id, amount);
        }
    }

    pub fn remove_item(&mut self, item_id: ItemId, amount: u32) {
        if self.inventory.contains_key(&item_id) {
            let value = self.inventory.get_mut(&item_id).unwrap();
            *value -= amount;
            if value == &0 {
                self.inventory.remove(&item_id);
            }
        } else {
            error!("Tried to remove an item from inventory that wasn't inside it: {item_id}");
        }
    }
}
