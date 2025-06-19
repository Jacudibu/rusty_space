use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_move_to_entity;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Entity, EventWriter, Query, Res};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{Asteroid, AsteroidMiner, Inventory};
use common::constants;
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
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

const MILLISECONDS_BETWEEN_UPDATES: Milliseconds = constants::ONE_SECOND_IN_MILLISECONDS;

enum TaskResult {
    Skip,
    Ongoing { mined_amount: u32 },
    Finished { mined_amount: u32 },
}

fn run(
    task: &mut ShipTask<MineAsteroid>,
    inventory: &mut Inventory,
    now: CurrentSimulationTimestamp,
    all_asteroids: &Query<(&mut Asteroid, &mut SimulationScale)>,
    mining_component: &AsteroidMiner,
    item_manifest: &ItemManifest,
) -> TaskResult {
    if now.has_not_passed(task.next_update.unwrap()) {
        return TaskResult::Skip;
    }

    let Ok((asteroid, _)) = all_asteroids.get(task.target.into()) else {
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
        task.next_update
            .unwrap()
            .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES);
        TaskResult::Ongoing { mined_amount }
    }
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgs<'w> {
    simulation_time: Res<'w, SimulationTime>,
    item_manifest: Res<'w, ItemManifest>,
}

#[derive(SystemParam)]
pub struct TaskUpdateRunnerArgsMut<'w, 's> {
    ships: Query<
        'w,
        's,
        (
            Entity,
            &'static mut ShipTask<MineAsteroid>,
            &'static mut Inventory,
            &'static AsteroidMiner,
        ),
    >,
    all_asteroids: Query<'w, 's, (&'static mut Asteroid, &'static mut SimulationScale)>,
    asteroid_was_fully_mined_event: EventWriter<'w, AsteroidWasFullyMinedEvent>,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for MineAsteroid {
    type Args = TaskUpdateRunnerArgs<'w>;
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<MineAsteroid>>::new()));
        let mined_asteroids = Arc::new(Mutex::new(Vec::<(AsteroidEntity, u32)>::new()));
        let now = args.simulation_time.now();

        args_mut.ships.par_iter_mut().for_each(
            |(entity, mut task, mut inventory, mining_component)| match run(
                &mut task,
                &mut inventory,
                now,
                &args_mut.all_asteroids,
                mining_component,
                &args.item_manifest,
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
            },
        );

        let Ok(mined_asteroids) = Arc::try_unwrap(mined_asteroids) else {
            todo!()
        };

        let batch = mined_asteroids.into_inner()?;
        if !batch.is_empty() {
            for (asteroid_entity, mined_amount) in batch {
                let Ok((mut asteroid, mut scale)) =
                    args_mut.all_asteroids.get_mut(asteroid_entity.into())
                else {
                    continue; // Must have already despawned
                };
                asteroid.ore_remaining -= mined_amount;
                scale.scale = asteroid.scale_depending_on_current_ore_volume();
                if asteroid.ore_remaining == 0 {
                    args_mut
                        .asteroid_was_fully_mined_event
                        .write(AsteroidWasFullyMinedEvent {
                            asteroid: asteroid_entity,
                            despawn_timer: asteroid.despawn_timestamp,
                        });
                }
            }
        }

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for MineAsteroid {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MineAsteroid>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
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

#[derive(SystemParam)]
pub struct TaskStartedArgs<'w> {
    simulation_time: Res<'w, SimulationTime>,
}

#[derive(SystemParam)]
pub struct TaskStartedArgsMut<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<MineAsteroid>>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for MineAsteroid {
    type Args = TaskStartedArgs<'w>;
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<Self>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let mut task = args_mut.all_ships_with_task.get_mut(event.entity.into())?;
        task.next_update = Some(
            args.simulation_time
                .now()
                .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES),
        );

        Ok(())
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for MineAsteroid {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for MineAsteroid {
    type Args = ();
    type ArgsMut = ();

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn skip_cancelled_while_active() -> bool {
        true
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for MineAsteroid {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}
