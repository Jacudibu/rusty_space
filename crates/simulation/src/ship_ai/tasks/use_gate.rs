use bevy::prelude::{
    Commands, CubicCurve, Entity, EventReader, EventWriter, Query, Res, Time, Vec2, With, error,
};
use std::sync::{Arc, Mutex};

use crate::ship_ai::TaskComponent;
use crate::ship_ai::ship_task::ShipTask;
use crate::ship_ai::task_result::TaskResult;
use crate::ship_ai::tasks::send_completion_events;
use common::components::ship_velocity::ShipVelocity;
use common::components::{Gate, InSector, Sector};
use common::events::task_events::TaskCompletedEvent;
use common::events::task_events::TaskStartedEvent;
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::ShipEntity;
use common::types::gate_traversal_state::GateTraversalState;
use common::types::ship_tasks::UseGate;
use common::{constants, interpolation};

impl TaskComponent for ShipTask<UseGate> {
    fn can_be_aborted() -> bool {
        false
    }
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

impl ShipTask<UseGate> {
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
        event_writer: EventWriter<TaskCompletedEvent<UseGate>>,
        time: Res<Time>,
        mut ships: Query<(Entity, &mut Self, &mut SimulationTransform)>,
        transit_curve_query: Query<&Gate>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskCompletedEvent<UseGate>>::new()));
        let delta_travel = time.delta_secs() / constants::SECONDS_TO_TRAVEL_THROUGH_GATE;

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut transform)| {
                match task.run(delta_travel, &mut transform, &transit_curve_query) {
                    TaskResult::Ongoing => {}
                    TaskResult::Finished | TaskResult::Aborted => task_completions
                        .lock()
                        .unwrap()
                        .push(TaskCompletedEvent::<UseGate>::new(entity.into())),
                }
            });

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskCompletedEvent<UseGate>>,
        mut all_ships_with_task: Query<(&Self, &mut ShipVelocity)>,
        mut all_sectors: Query<&mut Sector>,
    ) {
        for event in event_reader.read() {
            if let Ok((task, mut velocity)) = all_ships_with_task.get_mut(event.entity.into()) {
                all_sectors
                    .get_mut(task.exit_sector.into())
                    .unwrap()
                    .add_ship(
                        &mut commands,
                        task.exit_sector,
                        ShipEntity::from(event.entity),
                    );

                velocity.forward *= 0.5;
            } else {
                error!(
                    "Unable to find entity for UseGate task completion: {}",
                    event.entity
                );
            }
        }
    }

    pub fn on_task_started(
        mut commands: Commands,
        query: Query<&InSector, With<Self>>,
        mut triggers: EventReader<TaskStartedEvent<UseGate>>,
        mut all_sectors: Query<&mut Sector>,
    ) {
        for x in triggers.read() {
            let Ok(in_sector) = query.get(x.entity.into()) else {
                continue;
            };

            let mut sector = all_sectors.get_mut(in_sector.get().into()).unwrap();
            sector.remove_ship(&mut commands, ShipEntity::from(x.entity));
        }
    }

    pub(crate) fn cancel_task_inside_queue() {
        // Nothing needs to be done
    }

    pub(crate) fn abort_running_task() {
        panic!("UseGate cannot be aborted!");
    }
}
