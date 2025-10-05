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
use crate::utility::ship_task::ShipTask;
use crate::utility::task_result::TaskResult;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Commands, CubicCurve, Entity, Query, Res, Time, Vec2, With};
use common::components::ship_velocity::ShipVelocity;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{Gate, InSector, Sector};
use common::events::task_events::TaskCompletedEvent;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskStartedEvent};
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::ShipEntity;
use common::types::gate_traversal_state::GateTraversalState;
use common::types::ship_tasks::UseGate;
use common::{constants, interpolation};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

const GATE_ENTER_BLEND_THRESHOLD: f32 = 0.15;

#[inline]
fn blend_to_curve(
    origin: Vec2,
    transform: &mut SimulationTransform,
    progress: f32,
    curve: &CubicCurve<Vec2>,
) -> TaskResult {
    let target = interpolation::smooth_step(GATE_ENTER_BLEND_THRESHOLD);
    let blend_factor = progress / GATE_ENTER_BLEND_THRESHOLD;
    // TODO: Contemplate caching curve.position(target) result in gate on spawn
    // Could fill a little bubble around the ship during the one tick where there isn't much motion... warp bubbles!
    transform.translation = origin.lerp(curve.position(target), blend_factor * blend_factor);

    TaskResult::Ongoing
}

#[inline]
fn traverse_curve(
    transform: &mut SimulationTransform,
    progress: f32,
    curve: &CubicCurve<Vec2>,
) -> TaskResult {
    if progress <= 1.0 {
        let t = interpolation::smooth_step(progress);
        transform.translation = curve.position(t);

        TaskResult::Ongoing
    } else {
        TaskResult::Finished
    }
}

fn run(
    task: &mut ShipTask<UseGate>,
    delta_travel: f32,
    transform: &mut SimulationTransform,
    transit_curve_query: &Query<&Gate>,
) -> Result<TaskResult, BevyError> {
    task.progress += delta_travel;
    let curve = &transit_curve_query
        .get(task.enter_gate.into())?
        .transit_curve;

    let result = match task.traversal_state {
        GateTraversalState::JustCreated => {
            task.traversal_state = GateTraversalState::BlendingIntoMotion {
                origin: transform.translation,
            };

            blend_to_curve(transform.translation, transform, task.progress, curve)
        }
        GateTraversalState::BlendingIntoMotion { origin } => {
            if task.progress < GATE_ENTER_BLEND_THRESHOLD {
                blend_to_curve(origin, transform, task.progress, curve)
            } else {
                task.traversal_state = GateTraversalState::TraversingLine;
                traverse_curve(transform, task.progress, curve)
            }
        }
        GateTraversalState::TraversingLine => traverse_curve(transform, task.progress, curve),
    };

    Ok(result)
}

#[derive(SystemParam)]
pub struct TaskRunnerArgs<'w, 's> {
    time: Res<'w, Time>,
    transit_curve_query: Query<'w, 's, &'static Gate>,
}

#[derive(SystemParam)]
pub struct TaskRunnerArgsMut<'w, 's> {
    ships: Query<
        'w,
        's,
        (
            Entity,
            &'static mut ShipTask<UseGate>,
            &'static mut SimulationTransform,
        ),
    >,
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for UseGate {
    type Args = TaskRunnerArgs<'w, 's>;
    type ArgsMut = TaskRunnerArgsMut<'w, 's>;

    fn run_all_tasks(
        args: StaticSystemParam<Self::Args>,
        mut args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<UseGate>>>>, BevyError> {
        let args = args.deref();
        let args_mut = args_mut.deref_mut();

        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<UseGate>>::new()));
        let delta_travel = args.time.delta_secs() / constants::SECONDS_TO_TRAVEL_THROUGH_GATE;

        args_mut
            .ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut transform)| {
                match run(
                    &mut task,
                    delta_travel,
                    &mut transform,
                    &args.transit_curve_query,
                )
                .unwrap() // TODO: Error handling!
                {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<UseGate>::new(entity.into())),
                }
            });

        Ok(task_completions)
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for UseGate {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        message: &InsertTaskIntoQueueCommand<UseGate>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        todo!()
    }
}

#[derive(SystemParam)]
pub(crate) struct TaskStartedArgs<'w, 's> {
    in_sector: Query<'w, 's, &'static InSector, With<ShipTask<UseGate>>>,
}

#[derive(SystemParam)]
pub(crate) struct TaskStartedArgsMut<'w, 's> {
    commands: Commands<'w, 's>,
    all_sectors: Query<'w, 's, &'static mut Sector>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for UseGate {
    type Args = TaskStartedArgs<'w, 's>;
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<UseGate>,
        args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let in_sector = args.in_sector.get(event.entity.into())?;

        let mut sector = args_mut.all_sectors.get_mut(in_sector.get().into())?;
        sector.remove_ship(&mut args_mut.commands, event.entity);

        Ok(())
    }
}

impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for UseGate {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for UseGate {
    type Args = ();
    type ArgsMut = ();
}

#[derive(SystemParam)]
pub(crate) struct TaskCompletedArgsMut<'w, 's> {
    commands: Commands<'w, 's>,
    all_ships_with_task: Query<'w, 's, (&'static ShipTask<UseGate>, &'static mut ShipVelocity)>,
    all_sectors: Query<'w, 's, &'static mut Sector>,
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for UseGate {
    type Args = ();
    type ArgsMut = TaskCompletedArgsMut<'w, 's>;

    fn on_task_completed(
        event: &TaskCompletedEvent<UseGate>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let (task, mut velocity) = args_mut.all_ships_with_task.get_mut(event.entity.into())?;

        args_mut
            .all_sectors
            .get_mut(task.exit_sector.into())?
            .add_ship(
                &mut args_mut.commands,
                task.exit_sector,
                ShipEntity::from(event.entity),
            );

        velocity.forward *= 0.5;

        Ok(())
    }
}

impl<'w, 's> TaskMetaData<'w, 's, Self> for UseGate {
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
        task_metadata::get_entity_global_position(all_transforms, self.enter_gate.into())
    }
}
