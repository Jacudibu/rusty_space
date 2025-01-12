use crate::components::inventory::inventory_element::InventoryElement;
use crate::game_data::ItemId;
use crate::game_data::{ItemManifest, RecipeData};
use crate::utils::TradeIntent;
use bevy::log::error;
use bevy::prelude::{warn, Component};
use bevy::utils::HashMap;

#[derive(Component)]
pub struct Inventory {
    /// Total storage capacity for this inventory
    capacity: u32,

    /// Space that's occupied by items which are inside our inventory right now
    used_space: u32,

    /// Space that's reserved for incoming trade orders or production yields
    reserved_space: u32,

    /// Collection containing all items in this inventory.
    inventory: HashMap<ItemId, InventoryElement>,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            used_space: 0,
            reserved_space: 0,
            inventory: HashMap::new(),
        }
    }

    pub fn new_with_content(
        capacity: u32,
        content: Vec<(ItemId, u32)>,
        item_manifest: &ItemManifest,
    ) -> Self {
        let mut result = Self::new(capacity);

        let mut used_capacity = 0;
        for (item_id, amount) in content {
            result.inventory.insert(
                item_id,
                InventoryElement {
                    current: amount,
                    total: amount,
                    ..Default::default()
                },
            );
            used_capacity += item_manifest[item_id].size * amount;
        }

        result.used_space = used_capacity;
        result
    }

    #[inline]
    pub fn inventory(&self) -> &HashMap<ItemId, InventoryElement> {
        &self.inventory
    }

    #[inline]
    pub fn total_used_space(&self) -> u32 {
        self.used_space + self.reserved_space
    }

    #[inline]
    pub fn remaining_space_for(&self, item_id: &ItemId, item_manifest: &ItemManifest) -> u32 {
        self.remaining_space() / item_manifest[item_id].size
    }

    #[inline]
    pub fn remaining_space(&self) -> u32 {
        self.capacity - self.total_used_space()
    }

    pub fn ratio(&self) -> f32 {
        self.total_used_space() as f32 / self.capacity as f32
    }

    pub fn get(&self, item_id: &ItemId) -> Option<&InventoryElement> {
        self.inventory.get(item_id)
    }

    /// TODO: Extract
    pub fn create_order(
        &mut self,
        item_id: ItemId,
        intent: TradeIntent,
        amount: u32,
        item_manifest: &ItemManifest,
    ) {
        let storage_size = item_manifest[item_id].size * amount;
        if let Some(inventory) = self.inventory.get_mut(&item_id) {
            match intent {
                TradeIntent::Buy => {
                    self.reserved_space += storage_size;
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
                    self.reserved_space += storage_size;
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

    /// TODO: Extract
    pub fn complete_order(
        &mut self,
        item_id: ItemId,
        intent: TradeIntent,
        amount: u32,
        item_manifest: &ItemManifest,
    ) {
        let Some(inventory) = self.inventory.get_mut(&item_id) else {
            error!("Inventory Entry did not exist on order completion! (A)");
            return;
        };

        let storage_size = item_manifest[item_id].size * amount;
        match intent {
            TradeIntent::Buy => {
                self.used_space += storage_size;
                self.reserved_space -= storage_size;
                inventory.current += amount;
                inventory.planned_buying -= amount;
            }
            TradeIntent::Sell => {
                self.used_space -= storage_size;
                inventory.current -= amount;
                inventory.planned_selling -= amount;
            }
        }
    }

    /// Adds an item to the inventory, creating a new entry if one didn't already exist.
    pub fn add_item(&mut self, item: ItemId, amount: u32, item_manifest: &ItemManifest) {
        self.used_space += item_manifest[item].size * amount;
        if let Some(inventory_element) = self.inventory.get_mut(&item) {
            inventory_element.add(amount);
        } else {
            self.inventory.insert(item, InventoryElement::new(amount));
        }
    }

    /// Removes a specific item from the inventory.
    pub fn remove_item(&mut self, item: ItemId, amount: u32, item_manifest: &ItemManifest) {
        let Some(inventory_element) = self.inventory.get_mut(&item) else {
            warn!(
                "inventory entry did not exist when requesting removal for item {}!",
                item_manifest[item].name
            );
            return;
        };

        // TODO: Maybe delete the entry if it's now at 0? I guess in 90% of all cases they'd get re-added again fairly soonish
        self.used_space -= item_manifest[item].size * amount;
        inventory_element.remove(amount);
    }

    /// Reserves storage space for queued production yields.
    /// TODO: Extract
    pub fn reserve_storage_space_for_production_yield(
        &mut self,
        item_recipe: &RecipeData,
        multiplier: u32,
        item_manifest: &ItemManifest,
    ) {
        for output in &item_recipe.output {
            self.reserved_space += item_manifest[&output.item_id].size * multiplier;
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

    /// Adds the output materials for a recipe to this inventory and removes their storage reservation.
    /// TODO: Extract
    pub fn finish_production(
        &mut self,
        item_recipe: &RecipeData,
        multiplier: u32,
        item_manifest: &ItemManifest,
    ) {
        for output in &item_recipe.output {
            let product_size = item_manifest[&output.item_id].size * multiplier;
            self.used_space += product_size;
            self.reserved_space -= product_size;
            if let Some(inventory) = self.inventory.get_mut(&output.item_id) {
                inventory.current += output.amount * multiplier;
                inventory.planned_producing -= output.amount * multiplier;
            } else {
                warn!("Product inventory entry did not exist on production completion!");
                let item = InventoryElement {
                    total: output.amount * multiplier,
                    current: output.amount * multiplier,
                    ..Default::default()
                };
                self.inventory.insert(output.item_id, item);
            }
        }
    }
}
