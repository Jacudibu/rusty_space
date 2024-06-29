use crate::components::Inventory;
use crate::production::InventoryUpdateForProductionEvent;
use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks;
use crate::ship_ai::tasks::send_completion_events;
use crate::simulation_time::{SimulationSeconds, SimulationTime};
use crate::utils::ExchangeWareData;
use crate::utils::TradeIntent;
use bevy::prelude::{error, Commands, Component, Entity, EventReader, EventWriter, Query, Res};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Component)]
pub struct ExchangeWares {
    pub finishes_at: SimulationSeconds,
    pub target: Entity,
    pub data: ExchangeWareData,
}

impl ExchangeWares {
    fn run(&self, simulation_seconds: SimulationSeconds) -> TaskResult {
        if self.finishes_at > simulation_seconds {
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
    ) -> TaskResult {
        match all_storages.get_many_mut([this_entity, self.target]) {
            Ok([mut this_inv, mut other_inv]) => {
                match self.data {
                    ExchangeWareData::Buy(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Buy, amount);
                        other_inv.complete_order(item_id, TradeIntent::Sell, amount);
                    }
                    ExchangeWareData::Sell(item_id, amount) => {
                        this_inv.complete_order(item_id, TradeIntent::Sell, amount);
                        other_inv.complete_order(item_id, TradeIntent::Buy, amount);
                    }
                }
                event_writer.send(InventoryUpdateForProductionEvent::new(this_entity));
                event_writer.send(InventoryUpdateForProductionEvent::new(self.target));
                TaskResult::Finished
            }
            Err(e) => {
                error!(
                    "Failed to execute ware exchange between {this_entity} and {}: {:?}",
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
        let current_seconds = simulation_time.seconds();
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter()
            .for_each(|(entity, task)| match task.run(current_seconds) {
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
        mut all_ships_with_task: Query<(Entity, &mut TaskQueue, &Self)>,
        mut all_storages: Query<&mut Inventory>,
        mut event_writer: EventWriter<InventoryUpdateForProductionEvent>,
    ) {
        for event in event_reader.read() {
            if let Ok((entity, mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                task.complete(entity, &mut all_storages, &mut event_writer);

                tasks::remove_task_and_add_new_one::<Self>(&mut commands, entity, &mut queue);
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }
}
