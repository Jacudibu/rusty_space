use crate::constants;
use crate::sectors::InSector;
use crate::sectors::{AllGateConnections, AllSectors, GateId, SectorId};
use crate::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::ship_ai::task_queue::TaskQueue;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use crate::ship_ai::{tasks, MoveToEntity};
use crate::utils::interpolation;
use bevy::prelude::{
    error, Commands, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, Time,
    Transform,
};
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct UseGate {
    pub progress: f32,
    pub exit_sector: SectorId,
    pub exit_gate: GateId,
}

impl UseGate {
    fn run(
        &mut self,
        delta_travel: f32,
        transform: &mut Transform,
        all_gate_connections: &AllGateConnections,
    ) -> TaskResult {
        self.progress += delta_travel;
        if self.progress <= 1.0 {
            let t = interpolation::smooth_step(self.progress);
            let connection_data = all_gate_connections.get(&self.exit_gate).unwrap();
            transform.translation = connection_data.ship_curve.position(t);

            TaskResult::Ongoing
        } else {
            TaskResult::Finished
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &mut Self, &mut Transform)>,
        all_gate_connections: Res<AllGateConnections>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let delta_travel = time.delta_seconds() / constants::SECONDS_TO_TRAVEL_THROUGH_GATE;

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut transform)| {
                match task.run(delta_travel, &mut transform, &all_gate_connections) {
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
        mut all_sectors: ResMut<AllSectors>,
    ) {
        for event in event_reader.read() {
            if let Ok((entity, mut queue, task)) = all_ships_with_task.get_mut(event.entity) {
                all_sectors
                    .get_mut(&task.exit_sector)
                    .unwrap()
                    .add_ship(&mut commands, entity);

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
        query: Query<(&Self, &InSector)>,
        mut triggers: EventReader<TaskFinishedEvent<MoveToEntity>>,
        mut all_sectors: ResMut<AllSectors>,
    ) {
        for x in triggers.read() {
            let Ok((_, in_sector)) = query.get(x.entity) else {
                continue;
            };

            let sector = all_sectors.get_mut(&in_sector.get()).unwrap();
            sector.remove_ship(&mut commands, x.entity);
        }
    }
}
