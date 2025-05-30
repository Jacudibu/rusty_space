use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::task_creation::{TaskCreation, TaskCreationError, TaskCreationErrorReason};
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::{BevyError, Query};
use common::components::task_kind::TaskKind;
use common::components::{InSector, Sector};
use common::events::task_events::InsertTaskIntoQueueCommand;
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;
use std::ops::Deref;

#[derive(SystemParam)]
pub(crate) struct MoveToPositionArgs<'w, 's> {
    pub ships: Query<'w, 's, &'static InSector>,
    pub all_sectors: Query<'w, 's, &'static Sector>,
    pub all_transforms: Query<'w, 's, &'static SimulationTransform>,
}

impl TaskCreation<MoveToPosition, MoveToPositionArgs<'_, '_>> for MoveToPosition {
    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<MoveToPosition>,
        args: &StaticSystemParam<MoveToPositionArgs>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        let args = args.deref();
        let Ok(in_sector) = args.ships.get(event.entity) else {
            return Err(TaskCreationError {
                entity: event.entity,
                reason: TaskCreationErrorReason::ShipNotFound,
            }
            .into());
        };

        let mut new_tasks = VecDeque::default();

        let target_position = event.task_data.sector_position;
        if target_position.sector != in_sector.sector {
            let path = pathfinding::find_path(
                &args.all_sectors,
                &args.all_transforms,
                in_sector.sector,
                args.all_transforms.get(event.entity)?.translation,
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
}
