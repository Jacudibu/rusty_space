use crate::components::GasGiant;
use crate::simulation::prelude::{SimulationTime, TaskInsideQueue, TaskQueue};
use crate::simulation::ship_ai::tasks;
use crate::utils::PlanetEntity;
use bevy::prelude::{Commands, Component, Entity, Query, Res};

/// Intermediate task to reserve a spot inside an [`InteractionQueue`] attached to the [`target`].
///
/// Will always be immediately removed on execution, with two possible results depending on the queue's state:
///  - free: proceeding with the next task in this entity's local [`TaskQueue`]
///  - busy: spawning an [`AwaitingSignal`] Task
#[derive(Component)]
pub struct RequestAccess {
    target: PlanetEntity,
}

impl RequestAccess {
    pub fn new(target: PlanetEntity) -> Self {
        Self { target }
    }

    pub fn run_tasks(
        mut commands: Commands,
        mut all_ships_with_task: Query<(Entity, &Self, &mut TaskQueue)>,
        mut all_gas_giants: Query<&mut GasGiant>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();

        for (entity, task, mut task_queue) in all_ships_with_task.iter_mut() {
            let interaction_queue = &mut all_gas_giants
                .get_mut(task.target.into())
                .unwrap()
                .interaction_queue;

            if interaction_queue
                .try_start_interaction(&now, entity.into())
                .is_err()
            {
                task_queue.insert(1, TaskInsideQueue::AwaitingSignal);
            }

            tasks::remove_task_and_add_next_in_queue::<Self>(
                &mut commands,
                entity,
                &mut task_queue,
                now,
            );
        }
    }
}
