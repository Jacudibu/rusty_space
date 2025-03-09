use crate::simulation::prelude::{SimulationTime, TaskFinishedEvent, TaskQueue};
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
use crate::simulation::ship_ai::tasks;
use bevy::prelude::{error, Commands, Component, EventReader, Query, Res, With};

/// A ship with this component will be idle until it receives a Signal through an event.
#[derive(Component)]
pub struct AwaitingSignal {}

impl AwaitingSignal {
    /// Needs to run after any systems which may send TaskFinishedEvent<AwaitingSignal>, which is a bit nasty to manage
    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<&mut TaskQueue, With<Self>>,
        simulation_time: Res<SimulationTime>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity) {
                tasks::remove_task_and_add_next_in_queue::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
                    &mut task_started_event_writers,
                );
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }
}
