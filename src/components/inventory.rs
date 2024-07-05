use crate::game_data::ItemRecipe;
use crate::game_data::{ItemId, ItemRecipeElement};
use crate::utils::TradeIntent;
use bevy::log::error;
use bevy::prelude::{warn, Component};
use bevy::utils::{default, HashMap};

#[derive(Component)]
pub struct Inventory {
    pub capacity: u32,
    inventory: HashMap<ItemId, InventoryElement>,
}

#[derive(Default)]
pub struct InventoryElement {
    /// How much is actually inside our inventory, right now
    pub currently_available: u32,

    pub planned_buying: u32,
    pub planned_selling: u32,
    pub planned_producing: u32,

    /// Current + Buying + Selling + Producing
    pub total: u32,
}

impl InventoryElement {
    pub fn new(amount: u32) -> Self {
        Self {
            currently_available: amount,
            total: amount,
            ..default()
        }
    }

    pub fn add(&mut self, amount: u32) {
        self.currently_available += amount;
        self.total += amount;
    }

    pub fn remove(&mut self, amount: u32) {
        self.currently_available -= amount;
        self.total -= amount;
    }
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
                    total: amount,
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
                        planned_buying: amount,
                        ..Default::default()
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
            error!("Inventory Entry did not exist on order completion! (A)");
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

    /// Tests if there are enough items in stock to start a production run
    pub fn has_enough_items_in_inventory(
        &self,
        input: &Vec<ItemRecipeElement>,
        multiplier: u32,
    ) -> bool {
        for element in input {
            let Some(inventory) = self.inventory.get(&element.item_id) else {
                return false;
            };

            if inventory.currently_available - inventory.planned_selling
                < element.amount * multiplier
            {
                return false;
            }
        }

        true
    }

    /// Tests if there's enough storage space available to store all production yields.
    pub fn has_enough_storage_for_items(
        &self,
        output: &Vec<ItemRecipeElement>,
        multiplier: u32,
    ) -> bool {
        let mut total_used_storage = self.used();

        for element in output {
            total_used_storage += element.amount * multiplier;
            if let Some(inventory) = self.inventory.get(&element.item_id) {
                if element.amount * multiplier
                    + inventory.currently_available
                    + inventory.planned_buying
                    > self.capacity
                {
                    return false;
                }
            } else if total_used_storage > self.capacity {
                return false;
            }
        }

        true
    }

    /// Adds an item to the inventory, creating a new entry if one didn't exist yet.
    pub fn add_item(&mut self, item: ItemId, amount: u32) {
        if let Some(inventory) = self.inventory.get_mut(&item) {
            inventory.add(amount);
        } else {
            self.inventory.insert(item, InventoryElement::new(amount));
        }
    }

    /// Removes an item from the inventory.
    /// TODO: Maybe delete the entry if it's now at 0?
    pub fn remove_items(&mut self, items: &Vec<ItemRecipeElement>, multiplier: u32) {
        for item in items {
            let Some(inventory) = self.inventory.get_mut(&item.item_id) else {
                warn!("Ingredient inventory entry did not exist when requesting removal!");
                return;
            };

            inventory.remove(item.amount * multiplier);
        }
    }

    /// Removes the items required for a production run, and reserves inventory for the yields.
    pub fn reserve_storage_space_for_production_yield(
        &mut self,
        item_recipe: &ItemRecipe,
        multiplier: u32,
    ) {
        for output in &item_recipe.output {
            if let Some(inventory) = self.inventory.get_mut(&output.item_id) {
                inventory.planned_producing += output.amount * multiplier;
                inventory.total += output.amount * multiplier;
            } else {
                warn!("Product inventory entry did not exist when starting production!");
                let item = InventoryElement {
                    total: output.amount,
                    planned_producing: output.amount * multiplier,
                    ..Default::default()
                };
                self.inventory.insert(output.item_id, item);
            }
        }
    }

    pub fn finish_production(&mut self, item_recipe: &ItemRecipe, multiplier: u32) {
        for output in &item_recipe.output {
            if let Some(inventory) = self.inventory.get_mut(&output.item_id) {
                inventory.currently_available += output.amount * multiplier;
                inventory.planned_producing -= output.amount * multiplier;
            } else {
                warn!("Product inventory entry did not exist on production completion!");
                let item = InventoryElement {
                    total: output.amount * multiplier,
                    currently_available: output.amount * multiplier,
                    ..Default::default()
                };
                self.inventory.insert(output.item_id, item);
            }
        }
    }
}
