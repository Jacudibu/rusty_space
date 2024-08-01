use crate::components::{InteractionQueue, Inventory};
use crate::game_data::DEBUG_ITEM_ID_ORE;
use crate::simulation::prelude::{
    AwaitingSignal, CurrentSimulationTimestamp, Milliseconds, SimulationTime, SimulationTimestamp,
};
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::utils::PlanetEntity;
use bevy::log::error;
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res};
use std::sync::{Arc, Mutex};

pub const TIME_BETWEEN_UPDATES: Milliseconds = 1000;
pub const HARVESTED_AMOUNT_PER_UPDATE: u32 = 10;

enum TaskResult {
    Skip,
    Ongoing,
    Finished,
}

#[derive(Component)]
pub struct HarvestGas {
    pub target: PlanetEntity,
    next_update: SimulationTimestamp,
}

impl HarvestGas {
    pub fn new(target: PlanetEntity, now: CurrentSimulationTimestamp) -> Self {
        Self {
            target,
            next_update: now.add_milliseconds(TIME_BETWEEN_UPDATES),
        }
    }
}

impl HarvestGas {
    fn run(&mut self, inventory: &mut Inventory, now: CurrentSimulationTimestamp) -> TaskResult {
        if now.has_not_passed(self.next_update) {
            return TaskResult::Skip;
        }

        let harvested_amount =
            HARVESTED_AMOUNT_PER_UPDATE.min(inventory.capacity - inventory.used());

        inventory.add_item(DEBUG_ITEM_ID_ORE, harvested_amount);

        if inventory.used() == inventory.capacity {
            TaskResult::Finished
        } else {
            self.next_update.add_milliseconds(TIME_BETWEEN_UPDATES);
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory)>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let now = simulation_time.now();

        ships
            .par_iter_mut()
            .for_each(
                |(entity, mut task, mut inventory)| match task.run(&mut inventory, now) {
                    TaskResult::Skip => {}
                    TaskResult::Ongoing => {}
                    TaskResult::Finished => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                },
            );

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<(&mut TaskQueue, &Self)>,
        mut interaction_queues: Query<&mut InteractionQueue>,
        simulation_time: Res<SimulationTime>,
        mut signal_writer: EventWriter<TaskFinishedEvent<AwaitingSignal>>,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                interaction_queues
                    .get_mut(task.target.into())
                    .unwrap()
                    .finish_interaction(&mut signal_writer);

                tasks::remove_task_and_add_next_in_queue::<Self>(
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
}
