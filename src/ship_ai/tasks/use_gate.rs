use crate::constants::SHIP_LAYER;
use crate::sectors::{AllGateConnections, AllGates, AllSectors, GateConnection, GateId, InSector};
use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use crate::ship_ai::{tasks, MoveToEntity};
use crate::utils::SimulationTimestamp;
use crate::utils::{CurrentSimulationTimestamp, SimulationTime};
use bevy::prelude::{
    error, BuildChildren, BuildChildrenTransformExt, Commands, Component, Entity, EventReader,
    EventWriter, Query, Res, Transform,
};
use hexx::Hex;
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct UseGate {
    pub started_at: SimulationTimestamp,
    pub finishes_at: SimulationTimestamp,
    pub exit_sector: Hex,
    pub exit_gate: GateId,
}

impl UseGate {
    fn run(
        &self,
        now: CurrentSimulationTimestamp,
        transform: &mut Transform,
        all_gate_connections: &AllGateConnections,
        connection_components: &Query<&GateConnection>,
    ) -> TaskResult {
        if now.has_not_passed(self.finishes_at) {
            let max = self.finishes_at.milliseconds() - self.started_at.milliseconds();
            let passed = now.get() - self.started_at.milliseconds();

            let t = passed as f32 / max as f32;
            let t = -2.0 * t.powi(3) + 3.0 * t.powi(2);

            let connection_entity = all_gate_connections.get(&self.exit_gate).unwrap();
            let connection = connection_components
                .get(connection_entity.inner())
                .unwrap();

            transform.translation = connection_entity.evaluate_ship_position(connection, t);

            TaskResult::Ongoing
        } else {
            TaskResult::Finished
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &Self, &mut Transform)>,
        all_gate_connections: Res<AllGateConnections>,
        connection_components: Query<&GateConnection>,
    ) {
        let now = simulation_time.now();
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));

        ships
            .par_iter_mut()
            .for_each(|(entity, task, mut transform)| {
                match task.run(
                    now,
                    &mut transform,
                    &all_gate_connections,
                    &connection_components,
                ) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskFinishedEvent::<Self>::new(entity)),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<(Entity, &mut TaskQueue, &Self)>,
    ) {
        for event in event_reader.read() {
            if let Ok((entity, mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                commands
                    .entity(entity)
                    .insert(InSector::from(task.exit_sector));

                tasks::remove_task_and_add_new_one::<Self>(&mut commands, entity, &mut queue);
            } else {
                error!(
                    "Unable to find entity for task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub fn on_task_creation(
        mut commands: Commands,
        mut query: Query<&mut Self>,
        mut triggers: EventReader<TaskFinishedEvent<MoveToEntity>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();
        for x in triggers.read() {
            let Ok(mut created_component) = query.get_mut(x.entity) else {
                continue;
            };

            created_component.started_at = now.into();
            created_component.finishes_at = now.add_seconds(3);

            commands.entity(x.entity).remove::<InSector>();
        }
    }
}
