use crate::game_data::ItemId;
use crate::simulation::prelude::{CurrentSimulationTimestamp, SimulationTimestamp};
use crate::simulation::ship_ai::task_started_event::{
    AllTaskStartedEventWriters, TaskStartedEvent,
};
use crate::simulation::ship_ai::tasks;
use crate::simulation::ship_ai::tasks::MoveToEntity;
use crate::utils::{
    AsteroidEntity, ConstructionSiteEntity, ExchangeWareData, PlanetEntity, ShipEntity, TypedEntity,
};
use crate::utils::{GateEntity, SectorEntity};
use bevy::ecs::system::EntityCommands;

/// Defines a Task inside the [TaskQueue]. New task components can be created from these.
pub enum TaskInsideQueue {
    /// Indicates that our ship is waiting for an external entity (e.g. a station or the player) to signal the ship to continue with it next task.
    AwaitingSignal,
    Construct {
        target: ConstructionSiteEntity,
    },
    /// The ship will tell the provided entity that it wants to access it.
    /// Depending on how busy the target is, it will either tell us to go straight ahead and proceed with the next task or enter the ship into a queue, causing this task to be replaced by [TaskInsideQueue::AwaitingSignal].
    RequestAccess {
        target: TypedEntity,
    },
    DockAtEntity {
        target: TypedEntity,
    },
    Undock,
    ExchangeWares {
        target: TypedEntity,
        data: ExchangeWareData,
    },
    MoveToEntity {
        target: TypedEntity,
        /// If true, the ship will not slow down when approaching the target, meaning it will effectively fly right through it as the task completes.
        stop_at_target: bool,
        /// The desired distance to the target, in case the ship is not supposed to stop right on top of it but a bit earlier.
        distance_to_target: f32,
    },
    UseGate {
        enter_gate: GateEntity,
        exit_sector: SectorEntity,
    },
    MineAsteroid {
        target: AsteroidEntity,
        /// The amount of ore inside the asteroid which is reserved for our ship.
        reserved: u32,
    },
    HarvestGas {
        target: PlanetEntity,
        /// The [ItemId] for the gas which is supposed to be harvested. We need this since Gas Giants may contain multiple gases.
        gas: ItemId,
    },
}

impl TaskInsideQueue {
    pub fn create_and_insert_component(
        &self,
        entity: ShipEntity,
        entity_commands: &mut EntityCommands,
        now: CurrentSimulationTimestamp,
        task_started_event_writers: &mut AllTaskStartedEventWriters,
    ) {
        match self {
            TaskInsideQueue::ExchangeWares { target, data } => {
                task_started_event_writers
                    .exchange_wares
                    .send(TaskStartedEvent::new(entity));
                entity_commands.insert(tasks::ExchangeWares {
                    finishes_at: SimulationTimestamp::MAX,
                    target: *target,
                    data: *data,
                });
            }
            TaskInsideQueue::MoveToEntity {
                target,
                stop_at_target,
                distance_to_target: distance,
            } => {
                entity_commands.insert(MoveToEntity {
                    target: *target,
                    stop_at_target: *stop_at_target,
                    distance_to_target: *distance,
                });
            }
            TaskInsideQueue::UseGate {
                enter_gate,
                exit_sector,
            } => {
                task_started_event_writers
                    .use_gate
                    .send(TaskStartedEvent::new(entity));
                entity_commands.insert(tasks::UseGate {
                    progress: 0.0,
                    traversal_state: Default::default(),
                    exit_sector: *exit_sector,
                    enter_gate: *enter_gate,
                });
            }
            TaskInsideQueue::MineAsteroid { target, reserved } => {
                entity_commands.insert(tasks::MineAsteroid::new(*target, now, *reserved));
            }
            TaskInsideQueue::HarvestGas { target, gas } => {
                entity_commands.insert(tasks::HarvestGas::new(*target, *gas, now));
            }
            TaskInsideQueue::AwaitingSignal => {
                entity_commands.insert(tasks::AwaitingSignal {});
            }
            TaskInsideQueue::Construct { target } => {
                task_started_event_writers
                    .construct
                    .send(TaskStartedEvent::new(entity));
                entity_commands.insert(tasks::ConstructTaskComponent { target: *target });
            }
            TaskInsideQueue::RequestAccess { target } => {
                entity_commands.insert(tasks::RequestAccess::new(*target));
            }
            TaskInsideQueue::DockAtEntity { target } => {
                entity_commands.insert(tasks::DockAtEntity::new(*target));
            }
            TaskInsideQueue::Undock => {
                task_started_event_writers
                    .undock
                    .send(TaskStartedEvent::new(entity));
                entity_commands.insert(tasks::Undock::new());
            }
        }
    }
}
