use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, error};
use common::components::Inventory;
use common::events::inventory_update_for_production_event::InventoryUpdateForProductionEvent;
use common::events::task_events::{TaskCanceledEvent, TaskCompletedEvent, TaskStartedEvent};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTime};
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::ship_tasks::ExchangeWares;
use common::types::trade_intent::TradeIntent;
use std::sync::{Arc, Mutex};

impl TaskComponent for ShipTask<ExchangeWares> {
    fn can_be_aborted() -> bool {
        false
    }
}

impl ShipTask<ExchangeWares> {
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
                match self.exchange_data {
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
        event_writer: EventWriter<TaskCompletedEvent<ExchangeWares>>,
        simulation_time: Res<SimulationTime>,
        ships: Query<(Entity, &Self)>,
    ) {
        let now = simulation_time.now();
        let task_completions =
            Arc::new(Mutex::new(Vec::<TaskCompletedEvent<ExchangeWares>>::new()));

        ships
            .par_iter()
            .for_each(|(entity, task)| match task.run(now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<ExchangeWares>::new(entity.into())),
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut event_reader: EventReader<TaskCompletedEvent<ExchangeWares>>,
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
                    "Unable to find entity for ExchangeWares task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub fn on_task_started(
        mut query: Query<&mut Self>,
        mut finished_events: EventReader<TaskStartedEvent<ExchangeWares>>,
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

    pub(crate) fn cancel_task_inside_queue(
        mut events: EventReader<TaskCanceledEvent<ExchangeWares>>,
        mut inventories: Query<&mut Inventory>,
    ) {
        for event in events.read() {
            let exchange_data = &event.task_data.exchange_data;
            if let Ok(inventory) = inventories.get_mut(event.task_data.target.into()) {
                match exchange_data {
                    ExchangeWareData::Buy(item_id, amount) => {}
                    ExchangeWareData::Sell(item_id, amount) => {}
                }
            }

            if let Ok(inventory) = inventories.get_mut(event.entity.into()) {
                match exchange_data {
                    ExchangeWareData::Buy(item_id, amount) => {}
                    ExchangeWareData::Sell(item_id, amount) => {}
                }
            }
        }
    }
}
