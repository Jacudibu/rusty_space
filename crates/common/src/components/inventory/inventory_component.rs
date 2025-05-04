use crate::components::inventory::InventoryElement;
use crate::game_data::{ItemId, ItemManifest, RecipeData};
use crate::types::trade_intent::TradeIntent;
use bevy::log::error;
use bevy::platform::collections::HashMap;
use bevy::prelude::{Component, warn};

/// A component for storing all kinds of items.
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

    #[inline]
    pub fn inventory(&self) -> &HashMap<ItemId, InventoryElement> {
        &self.inventory
    }

    #[inline]
    pub fn total_used_space(&self) -> u32 {
        self.used_space + self.reserved_space
    }

    /// Calculates how many items with the size of the provided [item_id] still fit into this inventory.
    #[inline]
    pub fn remaining_space_for(&self, item_id: &ItemId, item_manifest: &ItemManifest) -> u32 {
        // TODO: This should probably take free reserved space into account?
        self.remaining_space() / item_manifest[item_id].size
    }

    #[inline]
    pub fn remaining_space(&self) -> u32 {
        self.capacity - self.total_used_space()
    }

    /// The percentage of used storage space.
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
                    inventory.planned_incoming += amount;
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
                        planned_incoming: amount,
                        ..Default::default()
                    };
                    self.inventory.insert(item_id, item);
                }
                TradeIntent::Sell => {
                    error!(
                        "How are we supposed to sell something if the item isn't even tracked inside our inventory yet?"
                    )
                }
            }
        }
    }

    /// Adjusts storage space reservations for a certain item
    pub fn set_production_reservation(
        &mut self,
        item_id: &ItemId,
        amount: u32,
        item_manifest: &ItemManifest,
    ) {
        self.set_reservation(item_id, amount, item_manifest, ReservationKind::Production)
    }

    /// Adjusts storage space reservations for a certain item
    pub fn set_purchase_reservation(
        &mut self,
        item_id: &ItemId,
        amount: u32,
        item_manifest: &ItemManifest,
    ) {
        self.set_reservation(item_id, amount, item_manifest, ReservationKind::Purchase)
    }

    fn set_reservation(
        &mut self,
        item_id: &ItemId,
        amount: u32,
        item_manifest: &ItemManifest,
        reservation_kind: ReservationKind,
    ) {
        let item = item_manifest.get_by_ref(item_id).unwrap();
        if !self.inventory.contains_key(&item.id) {
            self.add_item(item.id, 0, item_manifest);
        }
        let inventory_element = self.inventory.get_mut(&item.id).unwrap();
        let old_reserved = inventory_element.reserved();

        match reservation_kind {
            ReservationKind::Production => inventory_element.reserved_production += amount,
            ReservationKind::Purchase => inventory_element.reserved_buying += amount,
        }

        let new_reserved = inventory_element.reserved();

        if old_reserved > new_reserved {
            self.reserved_space -= (old_reserved - new_reserved) * item.size;
        } else {
            self.reserved_space += (new_reserved - old_reserved) * item.size;
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
                inventory.planned_incoming -= amount;
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
    pub fn reserve_storage_space_for_production_yield(&mut self, item_recipe: &RecipeData) {
        for output in &item_recipe.output {
            if let Some(inventory) = self.inventory.get_mut(&output.item_id) {
                inventory.add_incoming(output.amount);
            } else {
                warn!("Product inventory entry did not exist when starting production!");
                let mut item = InventoryElement::default();
                item.add_incoming(output.amount);
                self.inventory.insert(output.item_id, item);
            }
        }
    }

    /// Adds the output materials for a recipe to this inventory and removes their storage reservation.
    /// TODO: Extract
    pub fn finish_production(&mut self, item_recipe: &RecipeData) {
        for output in &item_recipe.output {
            if let Some(inventory) = self.inventory.get_mut(&output.item_id) {
                inventory.current += output.amount;
                inventory.planned_incoming -= output.amount;
            } else {
                warn!("Product inventory entry did not exist on production completion!");
                let mut item = InventoryElement::default();
                item.add(output.amount);
                self.inventory.insert(output.item_id, item);
            }
        }
    }
}

enum ReservationKind {
    Production,
    Purchase,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::create_id_constants;
    use crate::game_data::{RawItemData, RawItemManifest};
    use bevy::MinimalPlugins;
    use bevy::app::App;
    use bevy::prelude::{AssetApp, AssetPlugin, Image};
    use leafwing_manifest::manifest::Manifest;
    use rstest::{fixture, rstest};

    create_id_constants!(ItemId, ITEM_WITH_SIZE_1);
    create_id_constants!(ItemId, ITEM_WITH_SIZE_2);
    create_id_constants!(ItemId, ITEM_WITH_SIZE_25);
    create_id_constants!(ItemId, ITEM_WITH_SIZE_26);

    fn create_item(name: &str, size: u32) -> RawItemData {
        RawItemData {
            name: name.into(),
            icon: Default::default(),
            price_min: 10,
            price_max: 20,
            size,
        }
    }

    #[fixture]
    #[once]
    fn item_manifest() -> ItemManifest {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Image>();

        let world = app.world_mut();
        ItemManifest::from_raw_manifest(
            RawItemManifest {
                items: vec![
                    create_item(ITEM_WITH_SIZE_1_NAME, 1),
                    create_item(ITEM_WITH_SIZE_2_NAME, 2),
                    create_item(ITEM_WITH_SIZE_25_NAME, 25),
                    create_item(ITEM_WITH_SIZE_26_NAME, 26),
                ],
            },
            world,
        )
        .unwrap()
    }

    #[rstest]
    fn add_item(item_manifest: &ItemManifest) {
        let mut inventory = Inventory::new(25);
        assert_eq!(0.0, inventory.ratio());

        inventory.add_item(ITEM_WITH_SIZE_1_ID, 5, item_manifest);
        assert_eq!(5, inventory.used_space);
        assert_eq!(5, inventory.get(&ITEM_WITH_SIZE_1_ID).unwrap().current);

        inventory.add_item(ITEM_WITH_SIZE_2_ID, 10, item_manifest);
        assert_eq!(25, inventory.used_space);
        assert_eq!(10, inventory.get(&ITEM_WITH_SIZE_2_ID).unwrap().current);

        assert_eq!(1.0, inventory.ratio());
    }

    #[rstest]
    #[case(25, ITEM_WITH_SIZE_1_ID)]
    #[case(12, ITEM_WITH_SIZE_2_ID)]
    #[case(1, ITEM_WITH_SIZE_25_ID)]
    #[case(0, ITEM_WITH_SIZE_26_ID)]
    fn remaining_space_for(
        item_manifest: &ItemManifest,
        #[case] expected_result: u32,
        #[case] id: ItemId,
    ) {
        let inventory = Inventory::new(25);
        assert_eq!(
            expected_result,
            inventory.remaining_space_for(&id, item_manifest)
        );
    }
}
