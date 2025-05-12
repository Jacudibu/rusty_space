use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::tasks::{finish_interaction, send_completion_events};
use bevy::log::error;
use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res};
use common::components::interaction_queue::InteractionQueue;
use common::components::{GasHarvester, Inventory};
use common::constants;
use common::constants::BevyResult;
use common::events::task_events::{TaskCompletedEvent, TaskStartedEvent};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, Milliseconds, SimulationTime};
use common::types::ship_tasks::{AwaitingSignal, HarvestGas};
use std::sync::{Arc, Mutex};

const MILLISECONDS_BETWEEN_UPDATES: Milliseconds = constants::ONE_SECOND_IN_MILLISECONDS;

enum TaskResult {
    Skip,
    Ongoing,
    Finished,
}

impl TaskComponent for ShipTask<HarvestGas> {
    fn can_be_aborted() -> bool {
        true
    }
}

impl ShipTask<HarvestGas> {
    fn run(
        &mut self,
        inventory: &mut Inventory,
        now: CurrentSimulationTimestamp,
        harvesting_component: &GasHarvester,
        item_manifest: &ItemManifest,
    ) -> TaskResult {
        if now.has_not_passed(self.next_update.unwrap()) {
            return TaskResult::Skip;
        }

        let remaining_space = inventory.remaining_space_for(&self.gas, item_manifest);
        let harvested_amount = harvesting_component.amount_per_second.min(remaining_space);

        inventory.add_item(self.gas, harvested_amount, item_manifest);

        if remaining_space == harvested_amount {
            TaskResult::Finished
        } else {
            self.next_update
                .unwrap()
                .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES);
            TaskResult::Ongoing
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<HarvestGas>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory, &GasHarvester)>,
        item_manifest: Res<ItemManifest>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<HarvestGas>>::new()));
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
                        .push(TaskCompletedEvent::<HarvestGas>::new(entity.into())),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut event_reader: EventReader<TaskCompletedEvent<HarvestGas>>,
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
                    "Unable to find entity for HarvestGas task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub(crate) fn on_task_started(
        mut all_ships_with_task: Query<&mut Self>,
        mut started_tasks: EventReader<TaskStartedEvent<HarvestGas>>,
        simulation_time: Res<SimulationTime>,
    ) -> BevyResult {
        for event in started_tasks.read() {
            let mut task = all_ships_with_task.get_mut(event.entity.into())?;
            task.next_update = Some(
                simulation_time
                    .now()
                    .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES),
            );
        }

        Ok(())
    }
}
