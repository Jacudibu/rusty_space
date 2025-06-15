use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Entity, EventWriter, Query, warn};
use std::sync::{Arc, Mutex};

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

use crate::ship_ai::ship_task::ShipTask;
use common::components::interaction_queue::InteractionQueue;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::events::task_events::{
    AllTaskStartedEventWriters, TaskCompletedEvent, TaskStartedEvent,
};
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, MoveToSector, RequestAccess, ShipTaskData, Undock, UseGate,
};

/// Unwraps the provided event arc and writes them all at once into the respective [TaskCompletedEvent] event writer.
/// The main idea is that task runners can use par_iter_mut and then just pass any potential event completions in here.
pub fn send_completion_events<T: ShipTaskData>(
    mut event_writer: EventWriter<TaskCompletedEvent<T>>,
    task_completions: Arc<Mutex<Vec<TaskCompletedEvent<T>>>>,
) {
    match Arc::try_unwrap(task_completions) {
        Ok(task_completions) => {
            let batch = task_completions.into_inner().unwrap();
            if !batch.is_empty() {
                event_writer.write_batch(batch);
            }
        }
        Err(_) => {
            todo!()
        }
    }
}

/// Creates the Task Component for the first item in the queue to the provided entity.
/// Should be called by behaviors when transitioning away from idle states.
pub fn apply_new_task_queue(
    task_queue: &mut TaskQueue,
    commands: &mut Commands,
    entity: Entity,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    let mut commands = commands.entity(entity);
    apply_next_task(
        task_queue,
        entity.into(),
        &mut commands,
        task_started_event_writers,
    );
}

/// Applies the next task in the queue to be the new active task, or sets it to None if the queue is empty.
pub fn apply_next_task(
    task_queue: &mut TaskQueue,
    entity: ShipEntity,
    entity_commands: &mut EntityCommands,
    task_started_event_writers: &mut AllTaskStartedEventWriters,
) {
    task_queue.active_task = task_queue.queue.pop_front();
    let Some(next_task) = &task_queue.active_task else {
        return;
    };

    match next_task.clone() {
        TaskKind::ExchangeWares { data } => {
            task_started_event_writers
                .exchange_wares
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<ExchangeWares>::new(data));
        }
        TaskKind::MoveToEntity { data } => {
            entity_commands.insert(ShipTask::<MoveToEntity>::new(data));
        }
        TaskKind::MoveToPosition { data } => {
            entity_commands.insert(ShipTask::<MoveToPosition>::new(data));
        }
        TaskKind::MoveToSector { data } => {
            entity_commands.insert(ShipTask::<MoveToSector>::new(data));
        }
        TaskKind::UseGate { data } => {
            task_started_event_writers
                .use_gate
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<UseGate>::new(data));
        }
        TaskKind::MineAsteroid { data } => {
            task_started_event_writers
                .mine_asteroid
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<MineAsteroid>::new(data));
        }
        TaskKind::HarvestGas { data } => {
            task_started_event_writers
                .harvest_gas
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<HarvestGas>::new(data));
        }
        TaskKind::AwaitingSignal { data } => {
            entity_commands.insert(ShipTask::<AwaitingSignal>::new(data));
        }
        TaskKind::Construct { data } => {
            task_started_event_writers
                .construct
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<Construct>::new(data));
        }
        TaskKind::RequestAccess { data } => {
            entity_commands.insert(ShipTask::<RequestAccess>::new(data));
        }
        TaskKind::DockAtEntity { data } => {
            entity_commands.insert(ShipTask::<DockAtEntity>::new(data));
        }
        TaskKind::Undock { data } => {
            task_started_event_writers
                .undock
                .write(TaskStartedEvent::new(entity));
            entity_commands.insert(ShipTask::<Undock>::new(data));
        }
    }
}

/// Notify an [InteractionQueue] that the interaction has been finished, if it still exists.
#[inline]
pub fn finish_interaction(
    queue_entity: Entity,
    interaction_queues: &mut Query<&mut InteractionQueue>,
    signal_writer: &mut EventWriter<TaskCompletedEvent<AwaitingSignal>>,
) {
    let Ok(mut queue_entity) = interaction_queues.get_mut(queue_entity) else {
        warn!("Was unable to find queue entity in finish interaction!");
        return;
    };

    queue_entity.finish_interaction(signal_writer);
}

#[cfg(test)]
mod test {
    use crate::ship_ai::tasks::apply_new_task_queue;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{BevyError, Commands, Entity, Query};
    use common::components::task_kind::TaskKind;
    use common::components::task_queue::TaskQueue;
    use common::events::task_events::AllTaskStartedEventWriters;
    use common::types::entity_wrappers::{StationEntity, TypedEntity};
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

        app.world_mut().run_system_once(
            move |mut all_task_started_event_writers: AllTaskStartedEventWriters,
                  mut commands: Commands,
                  mut task_queue: Query<&mut TaskQueue>| {
                let mut task_queue = task_queue.single_mut().unwrap();
                apply_new_task_queue(
                    &mut task_queue,
                    &mut commands,
                    entity,
                    &mut all_task_started_event_writers,
                )
            },
        )?;

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
                    from: TypedEntity::Station(Entity::from_raw(1).into()),
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
                        from: TypedEntity::Station(Entity::from_raw(1).into()),
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
