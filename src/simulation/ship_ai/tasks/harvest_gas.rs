use crate::components::{GasHarvestingComponent, InteractionQueue, Inventory};
use crate::constants;
use crate::game_data::{ItemId, ItemManifest};
use crate::simulation::prelude::{
    AwaitingSignal, CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp, TaskComponent,
};
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::tasks::{finish_interaction, send_completion_events};
use crate::utils::PlanetEntity;
use bevy::log::error;
use bevy::prelude::{Component, Entity, EventReader, EventWriter, Query, Res};
use std::sync::{Arc, Mutex};

enum TaskResult {
    Skip,
    Ongoing,
    Finished,
}

/// Ships with this [TaskComponent] are currently harvesting gas from a gas giant.
#[derive(Component)]
pub struct HarvestGas {
    /// The entity of the gas giant from which we are harvesting.
    pub target: PlanetEntity,

    /// The gas which we are collecting
    pub gas: ItemId,

    /// A [SimulationTimestamp] to denote when the next inventory update occurs.
    next_update: SimulationTimestamp,
}

impl TaskComponent for HarvestGas {}

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
        item_manifest: &ItemManifest,
    ) -> TaskResult {
        if now.has_not_passed(self.next_update) {
            return TaskResult::Skip;
        }

        let remaining_space = inventory.remaining_space_for(&self.gas, item_manifest);
        let harvested_amount = harvesting_component.amount_per_second.min(remaining_space);

        inventory.add_item(self.gas, harvested_amount, item_manifest);

        if remaining_space == harvested_amount {
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
        item_manifest: Res<ItemManifest>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let now = simulation_time.now();

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut inventory, harvesting_component)| {
                match task.run(&mut inventory, now, harvesting_component, &item_manifest) {
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
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<&Self>,
        mut interaction_queues: Query<&mut InteractionQueue>,
        mut signal_writer: EventWriter<TaskFinishedEvent<AwaitingSignal>>,
    ) {
        for event in event_reader.read() {
            if let Ok(task) = all_ships_with_task.get_mut(event.entity) {
                finish_interaction(
                    task.target.into(),
                    &mut interaction_queues,
                    &mut signal_writer,
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
