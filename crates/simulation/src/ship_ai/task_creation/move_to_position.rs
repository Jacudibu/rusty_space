use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::task_creation::{TaskCreationError, TaskCreationErrorReason, apply_tasks};
use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::{BevyError, Commands, EventReader, Query};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, Sector};
use common::events::task_events::{AllTaskStartedEventWriters, InsertTaskIntoQueueCommand};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;

#[derive(SystemParam)]
pub(crate) struct MoveToPositionArgs<'w, 's> {
    pub ships: Query<'w, 's, (&'static mut TaskQueue, &'static InSector)>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
    pub commands: Commands<'w, 's>,
    pub all_task_started_event_writers: AllTaskStartedEventWriters<'w>,
}

pub(crate) fn move_to_position_command_listener(
    mut events: EventReader<InsertTaskIntoQueueCommand<MoveToPosition>>,
    mut args: MoveToPositionArgs,
) {
    for event in events.read() {
        if let Err(e) = handle_event(event, &mut args) {
            warn!(
                "Error whilst running move_to_position_command_listener: {:?}",
                e
            );
        }
    }
}

fn handle_event(
    event: &InsertTaskIntoQueueCommand<MoveToPosition>,
    args: &mut MoveToPositionArgs,
) -> Result<(), BevyError> {
    let Ok((mut queue, in_sector)) = args.ships.get_mut(event.entity) else {
        return Err(TaskCreationError {
            entity: event.entity,
            reason: TaskCreationErrorReason::ShipNotFound,
        }
        .into());
    };

    let new_tasks = create_tasks(event, &args.all_sectors, &args.all_transforms, in_sector)?;

    apply_tasks(
        new_tasks,
        event.insertion_mode,
        event.entity,
        &mut queue,
        &mut args.all_task_started_event_writers,
        &mut args.commands,
    );

    Ok(())
}

fn create_tasks(
    event: &InsertTaskIntoQueueCommand<MoveToPosition>,
    all_sectors: &Query<&Sector>,
    all_transforms: &Query<&SimulationTransform>,
    in_sector: &InSector,
) -> Result<VecDeque<TaskKind>, BevyError> {
    let mut new_tasks = VecDeque::default();

    let target_position = event.task_data.sector_position;
    if target_position.sector != in_sector.sector {
        let path = pathfinding::find_path(
            all_sectors,
            all_transforms,
            in_sector.sector,
            all_transforms.get(event.entity)?.translation,
            target_position.sector,
            Some(target_position.local_position),
        )
        .unwrap();

        create_tasks_to_follow_path(&mut new_tasks, path);
    }

    new_tasks.push_back(TaskKind::MoveToPosition {
        data: event.task_data.clone(),
    });
    Ok(new_tasks)
}
