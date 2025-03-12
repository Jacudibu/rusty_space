use bevy::prelude::{
    Commands, Component, CubicCurve, Entity, EventReader, EventWriter, Query, Res, Time, Vec2,
    With, error,
};
use std::sync::{Arc, Mutex};

use crate::components::{Gate, InSector, SectorComponent};
use crate::constants;
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::task_result::TaskResult;
use crate::simulation::ship_ai::task_started_event::{
    AllTaskStartedEventWriters, TaskStartedEvent,
};
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{GateEntity, SectorEntity};
use crate::utils::{ShipEntity, interpolation};

#[derive(Component)]
pub struct UseGate {
    pub progress: f32,
    pub traversal_state: GateTraversalState,
    pub enter_gate: GateEntity,
    pub exit_sector: SectorEntity,
}

#[derive(Default)]
pub enum GateTraversalState {
    #[default]
    JustCreated,
    BlendingIntoMotion {
        origin: Vec2,
    },
    TraversingLine,
}

const GATE_ENTER_BLEND_THRESHOLD: f32 = 0.15;

#[inline]
fn blend_to_curve(
    origin: Vec2,
    transform: &mut SimulationTransform,
    progress: f32,
    curve: &CubicCurve<Vec2>,
) -> TaskResult {
    let target = interpolation::smooth_step(GATE_ENTER_BLEND_THRESHOLD);
    let blend_factor = progress / GATE_ENTER_BLEND_THRESHOLD;
    // TODO: Contemplate caching curve.position(target) result in gate on spawn
    // Could fill a little bubble around the ship during the one tick where there isn't much motion... warp bubbles!
    transform.translation = origin.lerp(curve.position(target), blend_factor * blend_factor);

    TaskResult::Ongoing
}

#[inline]
fn traverse_curve(
    transform: &mut SimulationTransform,
    progress: f32,
    curve: &CubicCurve<Vec2>,
) -> TaskResult {
    if progress <= 1.0 {
        let t = interpolation::smooth_step(progress);
        transform.translation = curve.position(t);

        TaskResult::Ongoing
    } else {
        TaskResult::Finished
    }
}

impl UseGate {
    fn run(
        &mut self,
        delta_travel: f32,
        transform: &mut SimulationTransform,
        transit_curve_query: &Query<&Gate>,
    ) -> TaskResult {
        self.progress += delta_travel;
        let curve = &transit_curve_query
            .get(self.enter_gate.into())
            .unwrap()
            .transit_curve;

        match self.traversal_state {
            GateTraversalState::JustCreated => {
                self.traversal_state = GateTraversalState::BlendingIntoMotion {
                    origin: transform.translation,
                };

                blend_to_curve(transform.translation, transform, self.progress, curve)
            }
            GateTraversalState::BlendingIntoMotion { origin } => {
                if self.progress < GATE_ENTER_BLEND_THRESHOLD {
                    blend_to_curve(origin, transform, self.progress, curve)
                } else {
                    self.traversal_state = GateTraversalState::TraversingLine;
                    traverse_curve(transform, self.progress, curve)
                }
            }
            GateTraversalState::TraversingLine => traverse_curve(transform, self.progress, curve),
        }
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &mut Self, &mut SimulationTransform)>,
        transit_curve_query: Query<&Gate>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let delta_travel = time.delta_secs() / constants::SECONDS_TO_TRAVEL_THROUGH_GATE;

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut transform)| {
                match task.run(delta_travel, &mut transform, &transit_curve_query) {
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
        mut all_ships_with_task: Query<(&mut TaskQueue, &Self, &mut ShipVelocity)>,
        mut all_sectors: Query<&mut SectorComponent>,
        simulation_time: Res<SimulationTime>,
        mut task_started_event_writers: AllTaskStartedEventWriters,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok((mut queue, task, mut velocity)) = all_ships_with_task.get_mut(event.entity) {
                all_sectors
                    .get_mut(task.exit_sector.into())
                    .unwrap()
                    .add_ship(
                        &mut commands,
                        task.exit_sector,
                        ShipEntity::from(event.entity),
                    );

                velocity.forward *= 0.5;

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

    pub fn on_task_started(
        mut commands: Commands,
        query: Query<&InSector, With<Self>>,
        mut triggers: EventReader<TaskStartedEvent<Self>>,
        mut all_sectors: Query<&mut SectorComponent>,
    ) {
        for x in triggers.read() {
            let Ok(in_sector) = query.get(x.entity.into()) else {
                continue;
            };

            let mut sector = all_sectors.get_mut(in_sector.get().into()).unwrap();
            sector.remove_ship(&mut commands, ShipEntity::from(x.entity));
        }
    }
}
