use crate::components::{Asteroid, Inventory};
use crate::game_data::DEBUG_ITEM_ID_ORE;
use crate::simulation::asteroids::AsteroidWasFullyMinedEvent;
use crate::simulation::prelude::{
    CurrentSimulationTimestamp, Milliseconds, SimulationTime, SimulationTimestamp,
};
use crate::simulation::ship_ai::task_finished_event::TaskFinishedEvent;
use crate::simulation::ship_ai::task_queue::TaskQueue;
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::send_completion_events;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::AsteroidEntity;
use bevy::log::error;
use bevy::prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res, With};
use std::sync::{Arc, Mutex};

pub const TIME_BETWEEN_MINING_UPDATES: Milliseconds = 1000;
pub const MINED_AMOUNT_PER_UPDATE: u32 = 10;

enum TaskResult {
    Skip,
    Ongoing { mined_amount: u32 },
    Finished { mined_amount: u32 },
}

#[derive(Component)]
pub struct MineAsteroid {
    pub target: AsteroidEntity,
    next_update: SimulationTimestamp,
    reserved_ore_amount: u32,
}

impl MineAsteroid {
    pub fn new(target: AsteroidEntity, now: CurrentSimulationTimestamp, reserved: u32) -> Self {
        Self {
            target,
            next_update: now.add_milliseconds(TIME_BETWEEN_MINING_UPDATES),
            reserved_ore_amount: reserved,
        }
    }
}

impl MineAsteroid {
    fn run(
        &mut self,
        inventory: &mut Inventory,
        now: CurrentSimulationTimestamp,
        all_asteroids: &Query<(&mut Asteroid, &mut SimulationTransform)>,
    ) -> TaskResult {
        if now.has_not_passed(self.next_update) {
            return TaskResult::Skip;
        }

        let mined_amount = MINED_AMOUNT_PER_UPDATE
            .min(inventory.capacity - inventory.used())
            .min(self.reserved_ore_amount);

        inventory.add_item(DEBUG_ITEM_ID_ORE, mined_amount);
        self.reserved_ore_amount -= mined_amount;

        if self.reserved_ore_amount == 0 || inventory.used() == inventory.capacity {
            TaskResult::Finished { mined_amount }
        } else if self.did_target_despawn(all_asteroids) {
            TaskResult::Finished { mined_amount: 0 }
        } else {
            self.next_update
                .add_milliseconds(TIME_BETWEEN_MINING_UPDATES);
            TaskResult::Ongoing { mined_amount }
        }
    }

    fn did_target_despawn(
        &self,
        all_asteroids: &Query<(&mut Asteroid, &mut SimulationTransform)>,
    ) -> bool {
        all_asteroids
            .get(self.target.into())
            .unwrap()
            .0
            .state
            .is_despawned()
    }

    pub fn run_tasks(
        event_writer: EventWriter<TaskFinishedEvent<Self>>,
        simulation_time: Res<SimulationTime>,
        mut ships: Query<(Entity, &mut Self, &mut Inventory)>,
        mut all_asteroids: Query<(&mut Asteroid, &mut SimulationTransform)>,
        mut asteroid_was_fully_mined_event: EventWriter<AsteroidWasFullyMinedEvent>,
    ) {
        let task_completions = Arc::new(Mutex::new(Vec::<TaskFinishedEvent<Self>>::new()));
        let mined_asteroids = Arc::new(Mutex::new(Vec::<(AsteroidEntity, u32)>::new()));
        let now = simulation_time.now();

        ships
            .par_iter_mut()
            .for_each(|(entity, mut task, mut inventory)| {
                match task.run(&mut inventory, now, &all_asteroids) {
                    TaskResult::Skip => {}
                    TaskResult::Ongoing { mined_amount } => {
                        mined_asteroids
                            .lock()
                            .unwrap()
                            .push((task.target, mined_amount));
                    }
                    TaskResult::Finished { mined_amount } => {
                        mined_asteroids
                            .lock()
                            .unwrap()
                            .push((task.target, mined_amount));
                        task_completions
                            .lock()
                            .unwrap()
                            .push(TaskFinishedEvent::<Self>::new(entity))
                    }
                }
            });

        match Arc::try_unwrap(mined_asteroids) {
            Ok(mined_asteroids) => {
                let batch = mined_asteroids.into_inner().unwrap();
                if !batch.is_empty() {
                    for (asteroid_entity, mined_amount) in batch {
                        let (mut asteroid, mut transform) =
                            all_asteroids.get_mut(asteroid_entity.into()).unwrap();
                        asteroid.ore -= mined_amount;
                        transform.scale = asteroid.scale_depending_on_current_ore_volume();
                        if asteroid.ore == 0 {
                            asteroid_was_fully_mined_event.send(AsteroidWasFullyMinedEvent {
                                asteroid: asteroid_entity,
                                despawn_timer: asteroid.state.timestamp(),
                            });
                        }
                    }
                }
            }
            Err(_) => {
                todo!()
            }
        }

        send_completion_events(event_writer, task_completions);
    }

    pub fn complete_tasks(
        mut commands: Commands,
        mut event_reader: EventReader<TaskFinishedEvent<Self>>,
        mut all_ships_with_task: Query<&mut TaskQueue, With<Self>>,
        simulation_time: Res<SimulationTime>,
    ) {
        let now = simulation_time.now();

        for event in event_reader.read() {
            if let Ok(mut queue) = all_ships_with_task.get_mut(event.entity) {
                tasks::remove_task_and_add_new_one::<Self>(
                    &mut commands,
                    event.entity,
                    &mut queue,
                    now,
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
