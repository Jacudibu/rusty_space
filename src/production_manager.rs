use crate::components::{BuyOrders, Inventory, ProductionModule, SellOrders};
use crate::data::{GameData, ItemRecipe, RecipeId};
use crate::simulation_time::{SimulationSeconds, SimulationTime};
use bevy::prelude::{
    error, Entity, Event, EventReader, EventWriter, Mut, Query, Res, ResMut, Resource,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Keeps track of all ongoing production runs within in the ECS.
///
/// By using a binary heap to store references and timers to all ongoing production,
/// testing for finished production runs is O(1), and starting a new run is O(1)~ + O(log n).
#[derive(Resource)]
pub struct GlobalProductionState {
    elements: BinaryHeap<SingleProductionState>,
}

impl Default for GlobalProductionState {
    fn default() -> Self {
        Self {
            elements: BinaryHeap::with_capacity(200),
        }
    }
}

impl GlobalProductionState {
    fn insert(&mut self, value: SingleProductionState) {
        self.elements.push(value);
    }
}

// TODO: we need to keep track of the thing that's produced, otherwise Eq might be wrong as soon as we produce more than one item
#[derive(Eq, PartialEq)]
pub struct SingleProductionState {
    pub entity: Entity,
    pub recipe: RecipeId,
    pub finished_at: SimulationSeconds,
}

impl From<&ProductionStartedEvent> for SingleProductionState {
    fn from(value: &ProductionStartedEvent) -> Self {
        SingleProductionState {
            entity: value.entity,
            recipe: value.recipe_id,
            finished_at: value.finishes_at,
        }
    }
}

impl Ord for SingleProductionState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Inverted ordering so heap.max is our min element
        other.finished_at.cmp(&self.finished_at)
    }
}

impl PartialOrd for SingleProductionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Inverted ordering so heap.max is our min element
        Some(other.finished_at.cmp(&self.finished_at))
    }
}

#[derive(Event)]
pub struct ProductionStartedEvent {
    entity: Entity,
    recipe_id: RecipeId,
    finishes_at: SimulationSeconds,
}

pub fn on_production_started(
    mut global_production_state: ResMut<GlobalProductionState>,
    mut productions: EventReader<ProductionStartedEvent>,
) {
    for event in productions.read() {
        global_production_state.insert(SingleProductionState::from(event))
    }
}

pub fn update(
    simulation_time: Res<SimulationTime>,
    mut global_production_state: ResMut<GlobalProductionState>,
    game_data: Res<GameData>,
    mut production_start_event_writer: EventWriter<ProductionStartedEvent>,
    mut query: Query<(
        &mut ProductionModule,
        &mut Inventory,
        Option<&mut BuyOrders>,
        Option<&mut SellOrders>,
    )>,
) {
    let current = simulation_time.seconds();
    while let Some(next) = global_production_state.elements.peek() {
        if current < next.finished_at {
            break;
        }

        let next = global_production_state.elements.pop().unwrap();

        // TODO: Put this into another event?
        if let Ok((mut production, mut inventory, buy_orders, sell_orders)) =
            query.get_mut(next.entity)
        {
            let recipe = game_data.item_recipes.get(&next.recipe).unwrap();

            inventory.finish_production(recipe);
            if inventory.has_enough_items_to_start_production(recipe) {
                start_production(
                    &mut production_start_event_writer,
                    current,
                    next.entity,
                    &mut production,
                    &mut inventory,
                    recipe,
                );
            } else {
                production.current_run_finished_at = None;
            }

            update_orders(&inventory, buy_orders, sell_orders);
        } else {
            error!(
                "Was unable to trigger production finish for entity {}!",
                next.entity
            );
        }
    }
}

/// This event should be sent whenever an entity's inventory is being updated outside the production manager
///
/// More performant than querying with Changed<Inventory> since bevy won't need to iterate
/// through all entities matching the query every frame, plus it won't trigger itself recursively
/// ...the only risk is that we may forget to send it on inventory changes. What could go wrong?
#[derive(Event)]
pub struct TestIfEntityCanStartProductionEvent {
    entity: Entity,
}

impl TestIfEntityCanStartProductionEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

pub fn check_if_production_can_start_on_inventory_updates(
    simulation_time: Res<SimulationTime>,
    game_data: Res<GameData>,
    mut event_reader: EventReader<TestIfEntityCanStartProductionEvent>,
    mut production_start_event_writer: EventWriter<ProductionStartedEvent>,
    mut query: Query<(
        &mut ProductionModule,
        &mut Inventory,
        Option<&mut BuyOrders>,
        Option<&mut SellOrders>,
    )>,
) {
    let current = simulation_time.seconds();
    for event in event_reader.read() {
        let Ok((mut production, mut inventory, buy_orders, sell_orders)) =
            query.get_mut(event.entity)
        else {
            continue;
        };

        if production.current_run_finished_at.is_some() {
            continue;
        }

        let recipe = game_data.item_recipes.get(&production.recipe).unwrap();
        if inventory.has_enough_items_to_start_production(recipe) {
            start_production(
                &mut production_start_event_writer,
                current,
                event.entity,
                &mut production,
                &mut inventory,
                recipe,
            );

            update_orders(&inventory, buy_orders, sell_orders);
        }
    }
}

fn start_production(
    production_start_event_writer: &mut EventWriter<ProductionStartedEvent>,
    current: SimulationSeconds,
    entity: Entity,
    production: &mut Mut<ProductionModule>,
    inventory: &mut Mut<Inventory>,
    recipe: &ItemRecipe,
) {
    inventory.remove_items_to_start_production(recipe);

    let finish_timestamp = current + recipe.duration;
    production.current_run_finished_at = Some(finish_timestamp);

    production_start_event_writer.send(ProductionStartedEvent {
        entity,
        recipe_id: recipe.id,
        finishes_at: finish_timestamp,
    });
}

fn update_orders(
    inventory: &Inventory,
    buy_orders: Option<Mut<BuyOrders>>,
    sell_orders: Option<Mut<SellOrders>>,
) {
    if let Some(mut buy_orders) = buy_orders {
        buy_orders.update(inventory);
    }
    if let Some(mut sell_orders) = sell_orders {
        sell_orders.update(inventory);
    }
}
