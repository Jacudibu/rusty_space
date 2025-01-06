use crate::components::{GasHarvestingComponent, InteractionQueue, Inventory};
use crate::constants;
use crate::game_data::ItemId;
use crate::simulation::prelude::{
    AwaitingSignal, CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp,
};
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::{finish_interaction, send_completion_events};
use crate::utils::PlanetEntity;
use bevy::log::error;
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res};
use std::sync::{Arc, Mutex};

enum TaskResult {
    Skip,
    Ongoing,
    Finished,
}

#[derive(Component)]
pub struct HarvestGas {
    pub target: PlanetEntity,
    pub gas: ItemId,
    next_update: SimulationTimestamp,
}

impl HarvestGas {
    pub fn new(target: PlanetEntity, gas: ItemId, now: CurrentSimulationTimestamp) -> Self {
        Self {
            target,
            gas,
            next_update: now.add_milliseconds(constants::ONE_SECOND_IN_MILLISECONDS),
        }
    }
}

impl HarvestGas {
    fn run(
        &mut self,
        inventory: &mut Inventory,
        now: CurrentSimulationTimestamp,
        harvesting_component: &GasHarvestingComponent,
    ) -> TaskResult {
        if now.has_not_passed(self.next_update) {
            return TaskResult::Skip;
        }

        let harvested_amount = harvesting_component
            .amount_per_second
            .min(inventory.capacity - inventory.used());

        inventory.add_item(self.gas, harvested_amount);

        if inventory.used() == inventory.capacity {
            TaskResult::Finished
        } else {
            self.next_update
                .add_milliseconds(constants::ONE_SECOND_IN_MILLISECONDS);
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory, &GasHarvestingComponent)>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let now = simulation_time.now();

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut inventory, harvesting_component)| {
                match task.run(&mut inventory, now, harvesting_component) {
                    TaskResult::Skip => {}
                    TaskResult::Ongoing => {}
                    TaskResult::Finished => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                }
            });

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
                finish_interaction(
                    task.target.into(),
                    &mut interaction_queues,
                    &mut signal_writer,
                );

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
