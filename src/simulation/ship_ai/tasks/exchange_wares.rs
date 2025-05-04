use crate::components::Inventory;
use crate::game_data::ItemManifest;
use crate::simulation::prelude::TaskComponent;
use crate::simulation::production::InventoryUpdateForProductionEvent;
use crate::simulation::ship_ai::task_events::TaskCompletedEvent;
use crate::simulation::ship_ai::task_events::TaskStartedEvent;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::utils::ExchangeWareData;
use crate::utils::{TradeIntent, TypedEntity};
use bevy::prelude::{Component, Entity, EventReader, EventWriter, Query, Res, error};
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp};
use std::sync::{Arc, Mutex};

/// Ships with this [TaskComponent] are currently trading wares with the specified target entity.
/// (They basically just wait until a timer runs out and then transfer the items)
#[derive(Component)]
pub struct ExchangeWares {
    /// The [SimulationTimestamp] at which this transaction is supposed to finish.
    pub finishes_at: SimulationTimestamp,

    /// The entity representing our trading partner.
    pub target: TypedEntity,

    /// Further information on which wares are going to be exchanged.
    pub data: ExchangeWareData,
}

impl TaskComponent for ExchangeWares {
    fn can_be_aborted() -> bool {
        false
    }
}

impl ExchangeWares {
    fn run(&self, now: CurrentSimulationTimestamp) -> TaskResult {
        if now.has_not_passed(self.finishes_at) {
            TaskResult::Ongoing
        } else {
            TaskResult::Finished
        }
    }

    fn complete(
        &self,
        this_entity: Entity,
        all_storages: &mut Query<&mut Inventory>,
        event_writer: &mut EventWriter<InventoryUpdateForProductionEvent>,
        item_manifest: &ItemManifest,
    ) -> TaskResult {
        match all_storages.get_many_mut([this_entity, self.target.into()]) {
            Ok([mut this_inv, mut other_inv]) => {
                match self.data {
                    ExchangeWareData::Buy(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Buy, amount, item_manifest);
                        other_inv.complete_order(item_id, TradeIntent::Sell, amount, item_manifest);
                    }
                    ExchangeWareData::Sell(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Sell, amount, item_manifest);
                        other_inv.complete_order(item_id, TradeIntent::Buy, amount, item_manifest);
                    }
                }
                event_writer.write(InventoryUpdateForProductionEvent::new(this_entity));
                event_writer.write(InventoryUpdateForProductionEvent::new(self.target.into()));
                TaskResult::Finished
            }
            Err(e) => {
                error!(
                    "Failed to execute ware exchange between {this_entity} and {:?}: {:?}",
                    self.target, e
                );
                TaskResult::Aborted
            }
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        ships: Query<(Entity, &Self)>,
    ) {
        let now = simulation_time.now();
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Self>>::new()));

        ships
            .par_iter()
            .for_each(|(entity, task)| match task.run(now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<Self>::new(entity.into())),
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut event_reader: EventReader<TaskCompletedEvent<Self>>,
        mut all_ships_with_task: Query<&Self>,
        mut all_storages: Query<&mut Inventory>,
        mut event_writer: EventWriter<InventoryUpdateForProductionEvent>,
        item_manifest: Res<ItemManifest>,
    ) {
        for event in event_reader.read() {
            if let Ok(task) = all_ships_with_task.get_mut(event.entity.into()) {
                task.complete(
                    event.entity.into(),
                    &mut all_storages,
                    &mut event_writer,
                    &item_manifest,
                );
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub fn on_task_started(
        mut query: Query<&mut Self>,
        mut finished_events: EventReader<TaskStartedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();
        for x in finished_events.read() {
            let Ok(mut created_component) = query.get_mut(x.entity.into()) else {
                continue;
            };

            created_component.finishes_at = now.add_seconds(2);
        }
    }
}
