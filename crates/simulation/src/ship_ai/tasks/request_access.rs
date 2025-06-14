use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use bevy::prelude::{BevyError, Entity, EventWriter, Query};
use common::components::DockingBay;
use common::components::interaction_queue::{InteractionQueue, InteractionQueueResult};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::task_events::TaskCompletedEvent;
use common::types::ship_tasks::{AwaitingSignal, RequestAccess, RequestAccessGoal};

impl TaskComponent for ShipTask<RequestAccess> {
    fn can_be_cancelled_while_active() -> bool {
        true
    }
}

impl ShipTask<RequestAccess> {
    pub fn run_tasks(
        mut all_ships_with_task: Query<(Entity, &Self, &mut TaskQueue)>,
        mut all_interaction_queues: Query<&mut InteractionQueue>,
        mut all_docking_bays: Query<&mut DockingBay>,
        mut task_completions: EventWriter<TaskCompletedEvent<RequestAccess>>,
    ) -> BevyResult {
        for (entity, task, mut task_queue) in all_ships_with_task.iter_mut() {
            let result = match task.goal {
                RequestAccessGoal::Docking => {
                    Self::access_dock(entity, task, &mut all_docking_bays)
                }
                RequestAccessGoal::Undocking => {
                    Self::access_undock(entity, task, &mut all_docking_bays)
                }
                RequestAccessGoal::PlanetOrbit => {
                    Self::access_planet_orbit(entity, task, &mut all_interaction_queues)
                }
            }?;

            match result {
                InteractionQueueResult::ProceedImmediately => {}
                InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue => {
                    task_queue.push_front(TaskKind::AwaitingSignal {
                        data: AwaitingSignal { from: task.target },
                    });
                }
            }

            task_completions.write(TaskCompletedEvent::<RequestAccess>::new(entity.into()));
        }

        Ok(())
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }

    pub(crate) fn abort_running_task() {
        // Nothing needs to be done
        // TODO: Technically, we need to ensure that this didn't happen in between run_tasks and the task_completions reader
    }

    fn access_dock(
        entity: Entity,
        task: &ShipTask<RequestAccess>,
        all_docking_bays: &mut Query<&mut DockingBay>,
    ) -> Result<InteractionQueueResult, BevyError> {
        let Ok(mut docking_bay) = all_docking_bays.get_mut(task.target.into()) else {
            todo!("In case no entity to dock at was found, cancel task");
        };

        Ok(docking_bay.try_dock(entity.into()))
    }

    fn access_undock(
        entity: Entity,
        task: &ShipTask<RequestAccess>,
        all_docking_bays: &mut Query<&mut DockingBay>,
    ) -> Result<InteractionQueueResult, BevyError> {
        let Ok(mut docking_bay) = all_docking_bays.get_mut(task.target.into()) else {
            todo!("In case no entity to dock at was found, cancel task");
        };

        Ok(docking_bay.try_undock(entity.into()))
    }

    fn access_planet_orbit(
        entity: Entity,
        task: &ShipTask<RequestAccess>,
        all_interaction_queues: &mut Query<&mut InteractionQueue>,
    ) -> Result<InteractionQueueResult, BevyError> {
        let Ok(mut interaction_queue) = all_interaction_queues.get_mut(task.target.into()) else {
            panic!("Planets cannot be destroyed, so this should never happen!")
        };

        Ok(interaction_queue.try_start_interaction(entity.into()))
    }
}
