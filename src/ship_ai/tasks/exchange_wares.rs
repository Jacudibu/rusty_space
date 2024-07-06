use crate::components::Inventory;
use crate::physics::ShipVelocity;
use crate::production::InventoryUpdateForProductionEvent;
use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use crate::ship_ai::{tasks, MoveToEntity};
use crate::utils::TradeIntent;
use crate::utils::{CurrentSimulationTimestamp, SimulationTime};
use crate::utils::{ExchangeWareData, SimulationTimestamp};
use bevy::prelude::{error, Commands, Component, Entity, EventReader, EventWriter, Query, Res};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct ExchangeWares {
    pub finishes_at: SimulationTimestamp,
    pub target: Entity,
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
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                task.complete(event.entity, &mut all_storages, &mut event_writer);

                tasks::remove_task_and_add_new_one::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
                );
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }

    /// To avoid the O(a + n) query runtime for change detection, this just iterates through all relevant TaskFinishedEvents.
    /// Even in a busy session, there should always be *way, WAY* less of those than Entities.
    pub fn on_task_creation(
        mut query: Query<(&mut Self, &mut ShipVelocity)>,
        mut finished_events: EventReader<TaskFinishedEvent<MoveToEntity>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();
        for x in finished_events.read() {
            let Ok((mut created_component, mut velocity)) = query.get_mut(x.entity) else {
                continue;
            };

            created_component.finishes_at = now.add_seconds(2);

            // TODO: Remove this once docking is implemented
            velocity.forward = 0.0;
            velocity.angular = 0.0;
        }
    }
}
