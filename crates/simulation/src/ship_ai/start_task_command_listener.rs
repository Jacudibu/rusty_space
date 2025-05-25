use crate::ship_ai::create_tasks_following_path::create_tasks_to_follow_path;
use crate::ship_ai::tasks::{apply_new_task_queue, apply_next_task};
use bevy::prelude::{Commands, EventReader, Query, warn};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{InSector, Sector};
use common::constants::BevyResult;
use common::events::task_events::{AllTaskStartedEventWriters, InsertTaskIntoQueueCommand};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::MoveToPosition;

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

            create_tasks_to_follow_path(&mut queue, path);
        }

        queue.push_back(TaskKind::MoveToPosition {
            data: event.task_data.clone(),
        });

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
