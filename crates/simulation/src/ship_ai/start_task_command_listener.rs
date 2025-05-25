use crate::can_task_be_aborted;
use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::tasks::apply_next_task;
use bevy::prelude::{Commands, EventReader, Query, warn};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, Sector};
use common::constants::BevyResult;
use common::events::task_events::{
    AllTaskStartedEventWriters, InsertTaskIntoQueueCommand, TaskInsertionMode,
};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;
use std::collections::VecDeque;

pub(crate) fn move_to_position_command_listener(
    mut ships: Query<(&mut TaskQueue, &InSector)>,
    mut events: EventReader<InsertTaskIntoQueueCommand<MoveToPosition>>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&SimulationTransform>,
    mut commands: Commands,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) -> BevyResult {
    for event in events.read() {
        let Ok((mut queue, in_sector)) = ships.get_mut(event.entity) else {
            // The ship was probably destroyed
            warn!(
                "Unable to find ship {} to create tasks. Assuming it got destroyed?",
                event.entity
            );
            continue;
        };

        let mut new_tasks = VecDeque::default();

        let target_position = event.task_data.sector_position;
        if target_position.sector != in_sector.sector {
            let path = pathfinding::find_path(
                &all_sectors,
                &all_transforms,
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

        match event.insertion_mode {
            TaskInsertionMode::Append => {
                queue.append(&mut new_tasks);
            }
            TaskInsertionMode::Prepend => {
                if let Some(active_task) = queue.active_task.clone() {
                    if !can_task_be_aborted(&active_task) {
                        todo!(
                            "So, uh, this should probably just be skipped! Ideally before we do all the earlier calculation."
                        )
                    }

                    todo!(
                        "This isn't an abortion... and also not a cancellation... so, uh... yet another event to handle? Yaaay...~\
                         On the bright side, such task-delay thingies are inevitable in case a ship gets attacked and has to escape later on. 
                         We also need to remove the TaskComponent from the entity... fun!\
                         Best way is probably to leave the active task as-is and do all that in the task-delayed event handler. This is gonna be easier with task grouping."
                    );
                    queue.active_task = None;
                    queue.push_front(active_task);
                }

                for x in new_tasks.into_iter().rev() {
                    queue.push_front(x);
                }
            }
        };

        if queue.active_task.is_none() {
            apply_next_task(
                &mut queue,
                event.entity.into(),
                &mut commands.entity(event.entity),
                &mut all_task_started_event_writers,
            )
        }
    }

    Ok(())
}
