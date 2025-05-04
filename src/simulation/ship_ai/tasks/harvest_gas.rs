use crate::simulation::interaction_queue::InteractionQueue;
use crate::simulation::prelude::{AwaitingSignal, TaskComponent};
use crate::simulation::ship_ai::task_events::TaskCompletedEvent;
use crate::simulation::ship_ai::tasks::{finish_interaction, send_completion_events};
use crate::utils::CelestialEntity;
use bevy::log::error;
use bevy::prelude::{Component, Entity, EventReader, EventWriter, Query, Res};
use common::components::{GasHarvester, Inventory};
use common::constants;
use common::game_data::{ItemId, ItemManifest};
use common::simulation_time::{CurrentSimulationTimestamp, SimulationTime, SimulationTimestamp};
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
    pub target: CelestialEntity,

    /// The gas which we are collecting
    pub gas: ItemId,

    /// A [SimulationTimestamp] to denote when the next inventory update occurs.
    next_update: SimulationTimestamp,
}

impl TaskComponent for HarvestGas {
    fn can_be_aborted() -> bool {
        true
    }
}

impl HarvestGas {
    pub fn new(target: CelestialEntity, gas: ItemId, now: CurrentSimulationTimestamp) -> Self {
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
        harvesting_component: &GasHarvester,
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
        event_writer: EventWriter<TaskCompletedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory, &GasHarvester)>,
        item_manifest: Res<ItemManifest>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<Self>>::new()));
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
                        .push(TaskCompletedEvent::<Self>::new(entity.into())),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut event_reader: EventReader<TaskCompletedEvent<Self>>,
        mut all_ships_with_task: Query<&Self>,
        mut interaction_queues: Query<&mut InteractionQueue>,
        mut signal_writer: EventWriter<TaskCompletedEvent<AwaitingSignal>>,
    ) {
        for event in event_reader.read() {
            if let Ok(task) = all_ships_with_task.get_mut(event.entity.into()) {
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
