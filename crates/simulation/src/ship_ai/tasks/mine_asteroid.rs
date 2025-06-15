use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler, create_preconditions_and_move_to_entity,
};
use crate::ship_ai::tasks::send_completion_events;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventReader, EventWriter, Query, Res};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{Asteroid, AsteroidMiner, Inventory};
use common::constants;
use common::constants::BevyResult;
use common::events::asteroid_was_fully_mined_event::AsteroidWasFullyMinedEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCompletedEvent, TaskStartedEvent,
};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, Milliseconds, SimulationTime};
use common::simulation_transform::SimulationScale;
use common::types::entity_wrappers::AsteroidEntity;
use common::types::ship_tasks::MineAsteroid;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const MILLISECONDS_BETWEEN_UPDATES: Milliseconds = constants::ONE_SECOND_IN_MILLISECONDS;

enum TaskResult {
    Skip,
    Ongoing { mined_amount: u32 },
    Finished { mined_amount: u32 },
}

impl TaskComponent for ShipTask<MineAsteroid> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

impl ShipTask<MineAsteroid> {
    fn run(
        &mut self,
        inventory: &mut Inventory,
        now: CurrentSimulationTimestamp,
        all_asteroids: &Query<(&mut Asteroid, &mut SimulationScale)>,
        mining_component: &AsteroidMiner,
        item_manifest: &ItemManifest,
    ) -> TaskResult {
        if now.has_not_passed(self.next_update.unwrap()) {
            return TaskResult::Skip;
        }

        let Ok((asteroid, _)) = all_asteroids.get(self.target.into()) else {
            // Asteroid must have despawned
            return TaskResult::Finished { mined_amount: 0 };
        };

        let remaining_space = inventory.remaining_space_for(&asteroid.ore_item_id, item_manifest);
        let mined_amount = mining_component
            .amount_per_second
            .min(remaining_space)
            .min(asteroid.ore_remaining);

        inventory.add_item(asteroid.ore_item_id, mined_amount, item_manifest);

        if asteroid.ore_remaining == 0 || remaining_space == mined_amount {
            TaskResult::Finished { mined_amount }
        } else {
            self.next_update
                .unwrap()
                .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES);
            TaskResult::Ongoing { mined_amount }
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskCompletedEvent<MineAsteroid>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory, &AsteroidMiner)>,
        mut all_asteroids: Query<(&mut Asteroid, &mut SimulationScale)>,
        mut asteroid_was_fully_mined_event: EventWriter<AsteroidWasFullyMinedEvent>,
        item_manifest: Res<ItemManifest>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MineAsteroid>>::new()));
        let mined_asteroids = Arc::new(Mutex::new(Vec::<(AsteroidEntity, u32)>::new()));
        let now = simulation_time.now();

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut inventory, mining_component)| {
                match task.run(
                    &mut inventory,
                    now,
                    &all_asteroids,
                    mining_component,
                    &item_manifest,
                ) {
                    TaskResult::Skip => {}
                    TaskResult::Ongoing { mined_amount } => {
                        mined_asteroids
                            .lock()
                            .unwrap()
                            .push((task.target, mined_amount));
                    }
                    TaskResult::Finished { mined_amount } => {
                        if mined_amount > 0 {
                            mined_asteroids
                                .lock()
                                .unwrap()
                                .push((task.target, mined_amount));
                        }
                        task_completions
                            .lock()
                            .unwrap()
                            .push(TaskCompletedEvent::<MineAsteroid>::new(entity.into()))
                    }
                }
            });

        match Arc::try_unwrap(mined_asteroids) {
            Ok(mined_asteroids) => {
                let batch = mined_asteroids.into_inner().unwrap();
                if !batch.is_empty() {
                    for (asteroid_entity, mined_amount) in batch {
                        let Ok((mut asteroid, mut scale)) =
                            all_asteroids.get_mut(asteroid_entity.into())
                        else {
                            continue; // Must have already despawned
                        };
                        asteroid.ore_remaining -= mined_amount;
                        scale.scale = asteroid.scale_depending_on_current_ore_volume();
                        if asteroid.ore_remaining == 0 {
                            asteroid_was_fully_mined_event.write(AsteroidWasFullyMinedEvent {
                                asteroid: asteroid_entity,
                                despawn_timer: asteroid.despawn_timestamp,
                            });
                        }
                    }
                }
            }
            Err(_) => {
                todo!()
            }
        }

        send_completion_events(event_writer, task_completions);
    }

    pub(crate) fn on_task_started(
        mut all_ships_with_task: Query<&mut Self>,
        mut started_tasks: EventReader<TaskStartedEvent<MineAsteroid>>,
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

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done.
    }

    pub(crate) fn abort_running_task() {
        // Nothing needs to be done.
    }
}

#[derive(SystemParam)]
pub(crate) struct CreateMineAsteroidArgs {}

impl TaskCreationEventHandler<MineAsteroid, CreateMineAsteroidArgs> for MineAsteroid {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MineAsteroid>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &mut StaticSystemParam<CreateMineAsteroidArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let mut new_tasks = create_preconditions_and_move_to_entity(
            event.entity,
            event.task_data.target.into(),
            task_queue,
            general_pathfinding_args,
        )?;

        new_tasks.push_back(TaskKind::MineAsteroid {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}
