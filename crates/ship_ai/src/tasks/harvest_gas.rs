use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::task_metadata;
use crate::task_metadata::TaskMetaData;
use crate::tasks::finish_interaction;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_move_to_entity;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::math::Vec2;
use bevy::prelude::{BevyError, Entity, EventWriter, Query, Res};
use common::components::interaction_queue::InteractionQueue;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{GasHarvester, Inventory};
use common::constants;
use common::events::send_signal_event::SendSignalEvent;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::game_data::ItemManifest;
use common::simulation_time::{CurrentSimulationTimestamp, Milliseconds, SimulationTime};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::TypedEntity;
use common::types::ship_tasks;
use common::types::ship_tasks::{HarvestGas, RequestAccessGoal};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

const MILLISECONDS_BETWEEN_UPDATES: Milliseconds = constants::ONE_SECOND_IN_MILLISECONDS;

enum TaskResult {
    Skip,
    Ongoing,
    Finished,
}

fn run(
    task: &mut ShipTask<HarvestGas>,
    inventory: &mut Inventory,
    now: CurrentSimulationTimestamp,
    harvesting_component: &GasHarvester,
    item_manifest: &ItemManifest,
) -> TaskResult {
    if now.has_not_passed(task.next_update.unwrap()) {
        return TaskResult::Skip;
    }

    let remaining_space = inventory.remaining_space_for(&task.gas, item_manifest);
    let harvested_amount = harvesting_component.amount_per_second.min(remaining_space);

    inventory.add_item(task.gas, harvested_amount, item_manifest);

    if remaining_space == harvested_amount {
        TaskResult::Finished
    } else {
        task.next_update
            .unwrap()
            .add_milliseconds(MILLISECONDS_BETWEEN_UPDATES);
        TaskResult::Ongoing
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
            &'static mut ShipTask<HarvestGas>,
            &'static mut Inventory,
            &'static GasHarvester,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for HarvestGas {
    type Args = TaskUpdateRunnerArgs<'w>;
    type ArgsMut = TaskUpdateRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<HarvestGas>>::new()));
        let now = args.simulation_time.now();

        args_mut.ships.par_iter_mut().for_each(
            |(entity, mut task, mut inventory, harvesting_component)| match run(
                &mut task,
                &mut inventory,
                now,
                harvesting_component,
                &args.item_manifest,
            ) {
                TaskResult::Skip => {}
                TaskResult::Ongoing => {}
                TaskResult::Finished => task_completions
                    .lock()
                    .unwrap()
                    .push(TaskCompletedEvent::<HarvestGas>::new(entity.into())),
            },
        );

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, HarvestGas> for HarvestGas {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<HarvestGas>,
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

        new_tasks.push_back(TaskKind::RequestAccess {
            data: ship_tasks::RequestAccess {
                target: TypedEntity::Celestial(event.task_data.target),
                goal: RequestAccessGoal::PlanetOrbit,
            },
        });
        new_tasks.push_back(TaskKind::HarvestGas {
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
    all_ships_with_task: Query<'w, 's, &'static mut ShipTask<HarvestGas>>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for HarvestGas {
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

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for HarvestGas {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

#[derive(SystemParam)]
pub struct TaskCancellationWhileActiveArgsMut<'w, 's> {
    interaction_queues: Query<'w, 's, &'static mut InteractionQueue>,
    signal_writer: EventWriter<'w, SendSignalEvent>,
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for HarvestGas {
    type Args = ();
    type ArgsMut = TaskCancellationWhileActiveArgsMut<'w, 's>;

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        finish_interaction(
            event.task_data.target.into(),
            &mut args_mut.interaction_queues,
            &mut args_mut.signal_writer,
        )
    }
}

#[derive(SystemParam)]
pub struct TaskCompletedArgsMut<'w, 's> {
    all_ships_with_task: Query<'w, 's, &'static ShipTask<HarvestGas>>,
    interaction_queues: Query<'w, 's, &'static mut InteractionQueue>,
    signal_writer: EventWriter<'w, SendSignalEvent>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for HarvestGas {
    type Args = ();
    type ArgsMut = TaskCompletedArgsMut<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let task = args_mut.all_ships_with_task.get_mut(event.entity.into())?;

        finish_interaction(
            task.target.into(),
            &mut args_mut.interaction_queues,
            &mut args_mut.signal_writer,
        )
    }
}

impl<'w, 's> TaskMetaData<'w, 's, Self> for HarvestGas {
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
        task_metadata::get_entity_global_position(all_transforms, self.target.into())
    }
}
