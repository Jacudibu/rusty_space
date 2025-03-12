use crate::components::Inventory;
use crate::game_data::ItemManifest;
use crate::simulation::prelude::{CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp};
use crate::simulation::production::InventoryUpdateForProductionEvent;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::task_started_event::{
    AllTaskStartedEventWriters, TaskStartedEvent,
};
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::utils::ExchangeWareData;
use crate::utils::{TradeIntent, TypedEntity};
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res, error};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct ExchangeWares {
    pub finishes_at: SimulationTimestamp,
    pub target: TypedEntity,
    pub data: ExchangeWareData,
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
                event_writer.send(InventoryUpdateForProductionEvent::new(this_entity));
                event_writer.send(InventoryUpdateForProductionEvent::new(self.target.into()));
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
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        ships: Query<(Entity, &Self)>,
    ) {
        let now = simulation_time.now();
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter()
            .for_each(|(entity, task)| match task.run(now) {
                TaskResult::Ongoing => {}
                TaskResult::Finished | TaskResult::Aborted => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskFinishedEvent::<Self>::new(entity)),
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<(&mut TaskQueue, &Self)>,
        mut all_storages: Query<&mut Inventory>,
        mut event_writer: EventWriter<InventoryUpdateForProductionEvent>,
        simulation_time: Res<SimulationTime>,
        item_manifest: Res<ItemManifest>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                task.complete(
                    event.entity,
                    &mut all_storages,
                    &mut event_writer,
                    &item_manifest,
                );

                tasks::remove_task_and_add_next_in_queue::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
                    &mut task_started_event_writers,
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
