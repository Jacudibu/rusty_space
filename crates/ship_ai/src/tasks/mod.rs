use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Entity, MessageWriter, Query};

mod awaiting_signal;
mod construct;
mod dock_at_entity;
mod exchange_wares;
mod harvest_gas;
mod mine_asteroid;
mod move_to_entity;
mod move_to_position;
mod move_to_sector;
mod request_access;
mod undock;
mod use_gate;

use crate::task_kind_extension::TaskKindExtInternal;
use common::components::interaction_queue::InteractionQueue;
use common::components::task_queue::TaskQueue;
use common::constants::BevyResult;
use common::events::send_signal_event::SendSignalEvent;
use common::events::task_events::AllTaskStartedMessageWriters;
use common::types::entity_wrappers::ShipEntity;

/// Applies the next task in the queue to be the new active task, or sets it to None if the queue is empty.
pub fn apply_next_task(
    task_queue: &mut TaskQueue,
    entity: ShipEntity,
    entity_commands: &mut EntityCommands,
    task_started_event_writers: &mut AllTaskStartedMessageWriters,
) {
    task_queue.active_task = task_queue.queue.pop_front();
    let Some(next_task) = &task_queue.active_task else {
        return;
    };

    next_task.add_task_to_entity(entity_commands);
    task_started_event_writers.write_event(entity, next_task);
}

/// Notify an [InteractionQueue] that the interaction has been finished, if it still exists.
#[inline]
pub fn finish_interaction(
    queue_entity: Entity,
    interaction_queues: &mut Query<&mut InteractionQueue>,
    signal_writer: &mut MessageWriter<SendSignalEvent>,
) -> BevyResult {
    let mut queue_entity = interaction_queues.get_mut(queue_entity)?;
    queue_entity.finish_interaction(signal_writer);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::tasks::apply_next_task;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{BevyError, Commands, Entity, Query};
    use common::components::task_kind::TaskKind;
    use common::components::task_queue::TaskQueue;
    use common::events::task_events::AllTaskStartedMessageWriters;
    use common::types::entity_wrappers::TypedEntity;
    use common::types::ship_tasks::{MoveToEntity, Undock};
    use std::collections::VecDeque;

    fn apply_task_queue(task_queue: TaskQueue) -> Result<TaskQueue, BevyError> {
        let mut app = App::new();
        test_utils::add_all_event_writers_for_tests(&mut app);

        let entity = {
            let world = app.world_mut();
            let mut commands = world.commands();
            commands.spawn(task_queue).id()
        };

        app.update();

        app.world_mut()
            .run_system_once(
                move |mut all_task_started_event_writers: AllTaskStartedMessageWriters,
                      mut commands: Commands,
                      mut task_queue: Query<&mut TaskQueue>| {
                    let mut task_queue = task_queue.single_mut().unwrap();
                    apply_next_task(
                        &mut task_queue,
                        entity.into(),
                        &mut commands.entity(entity),
                        &mut all_task_started_event_writers,
                    )
                },
            )
            .unwrap();

        let result = app
            .world_mut()
            .query::<&mut TaskQueue>()
            .single(app.world_mut())?;

        Ok(TaskQueue {
            active_task: result.active_task.clone(),
            queue: result.queue.clone(),
        })
    }

    #[test]
    fn applying_empty_queue_should_set_active_task_to_none() -> Result<(), BevyError> {
        let to_test = apply_task_queue(TaskQueue {
            queue: VecDeque::default(),
            active_task: Some(TaskKind::Undock {
                data: Undock {
                    start_position: None,
                    from: TypedEntity::Station(test_utils::mock_entity_id(1)),
                },
            }),
        })?;

        assert!(to_test.active_task.is_none());
        Ok(())
    }

    #[test]
    fn applying_filled_queue_should_set_active_task_to_first() -> Result<(), BevyError> {
        let to_test = apply_task_queue(TaskQueue {
            queue: vec![
                TaskKind::Undock {
                    data: Undock {
                        start_position: None,
                        from: TypedEntity::Station(test_utils::mock_entity_id(1)),
                    },
                },
                TaskKind::MoveToEntity {
                    data: MoveToEntity {
                        desired_distance_to_target: 0.0,
                        target: TypedEntity::Ship(test_utils::mock_entity_id(1)),
                        stop_at_target: true,
                    },
                },
            ]
            .into(),
            active_task: None,
        })?;

        assert!(matches!(to_test.active_task, Some(TaskKind::Undock { .. })));
        assert!(matches!(
            to_test.queue.front(),
            Some(TaskKind::MoveToEntity { .. })
        ));
        Ok(())
    }
}
