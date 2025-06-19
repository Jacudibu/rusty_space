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
use bevy::prelude::{BevyError, EventWriter, Query, Res, error};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{ConstructionSite, Ship};
use common::constants::BevyResult;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::session_data::ShipConfigurationManifest;
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::Construct;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

/// Registers a ship as an active worker for the provided [ConstructionSite].
pub fn register_ship(site: &mut ConstructionSite, entity: ShipEntity, build_power: u32) {
    site.total_build_power_of_ships += build_power;
    if let Some(old_value) = site.construction_ships.insert(entity, build_power) {
        site.total_build_power_of_ships -= old_value;
    }
}

/// Removes a ship registration from the provided [ConstructionSite]
pub fn deregister_ship(site: &mut ConstructionSite, entity: ShipEntity) {
    if let Some(build_power) = site.construction_ships.remove(&entity) {
        site.total_build_power_of_ships -= build_power;
    }
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn run_all_tasks(
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        panic!("This should never be called")
    }

    fn update(
        _event_writer: EventWriter<TaskCompletedEvent<Self>>,
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        Ok(())
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<Construct>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        // let args = args.deref_mut();

        let mut new_tasks = create_preconditions_and_move_to_entity(
            event.entity,
            event.task_data.target.into(),
            task_queue,
            general_pathfinding_args,
        )?;

        new_tasks.push_back(TaskKind::Construct {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}

#[derive(SystemParam)]
pub struct TaskStartedArgsMut<'w, 's> {
    construction_tasks: Query<'w, 's, (&'static ShipTask<Construct>, &'static Ship)>,
    construction_sites: Query<'w, 's, &'static mut ConstructionSite>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let (task, ship) = args_mut.construction_tasks.get(event.entity.into())?;
        let mut construction_site = args_mut.construction_sites.get_mut(task.target.into())?;

        let Some(ship_config) = args_mut.ship_configurations.get_by_id(&ship.config_id()) else {
            error!(
                "Attempted to start construction task on ship without existing configuration: {:?}",
                event.entity
            );
            return Ok(());
        };

        let Some(build_power) = ship_config.computed_stats.build_power else {
            error!(
                "Attempted to start construction task on ship without build power: {:?}",
                event.entity
            );
            return Ok(());
        };

        register_ship(&mut construction_site, event.entity, build_power);
        Ok(())
    }
}
impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

#[derive(SystemParam)]
pub struct TaskCancellationForActiveTaskArgsMut<'w, 's> {
    construction_sites: Query<'w, 's, &'static mut ConstructionSite>,
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = TaskCancellationForActiveTaskArgsMut<'w, 's>;

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let mut site = args_mut
            .construction_sites
            .get_mut(event.task_data.target.into())?;

        deregister_ship(&mut site, event.entity);
        Ok(())
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}
