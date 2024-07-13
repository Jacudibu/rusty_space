use crate::components::Sector;
use crate::game_data::ItemId;
use crate::persistence::persistent_entity_id::{
    PersistentAsteroidId, PersistentEntityId, PersistentGateId,
};
use crate::physics::ShipVelocity;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::utils::ExchangeWareData;
use bevy::math::{EulerRot, Vec2};
use bevy::prelude::{Query, Transform};
use hexx::Hex;

pub enum ExchangeWareSaveData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}

impl From<&ExchangeWareData> for ExchangeWareSaveData {
    fn from(value: &ExchangeWareData) -> Self {
        match value {
            ExchangeWareData::Buy(item, amount) => Self::Buy(*item, *amount),
            ExchangeWareData::Sell(item, amount) => Self::Sell(*item, *amount),
        }
    }
}

pub enum TaskSaveData {
    ExchangeWares {
        target: PersistentEntityId,
        data: ExchangeWareSaveData,
    },
    MoveToEntity {
        target: PersistentEntityId,
        stop_at_target: bool,
    },
    UseGate {
        enter_gate: PersistentGateId,
        exit_sector: Hex,
    },
    MineAsteroid {
        target: PersistentAsteroidId,
        reserved: u32,
    },
}

impl TaskSaveData {
    pub fn from(task: &TaskInsideQueue) -> Self {
        match task {
            TaskInsideQueue::ExchangeWares { target, data } => Self::ExchangeWares {
                target: todo!(),
                data: data.into(),
            },
            TaskInsideQueue::MoveToEntity {
                target,
                stop_at_target,
            } => Self::MoveToEntity {
                target: todo!(),
                stop_at_target: *stop_at_target,
            },
            TaskInsideQueue::UseGate {
                enter_gate,
                exit_sector,
            } => Self::UseGate {
                enter_gate: todo!(),
                exit_sector: todo!(),
            },
            TaskInsideQueue::MineAsteroid { target, reserved } => Self::MineAsteroid {
                target: todo!(),
                reserved: *reserved,
            },
        }
    }
}

pub struct ShipSaveData {
    position: Vec2,
    forward_velocity: f32,
    rotation: f32,
    angular_velocity: f32,
    task_queue: Vec<TaskSaveData>,
}

impl ShipSaveData {
    pub fn from(
        (transform, task_queue, velocity): (&Transform, &TaskQueue, &ShipVelocity),
    ) -> Self {
        Self {
            position: transform.translation.truncate(),
            forward_velocity: velocity.forward,
            rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
            angular_velocity: velocity.angular,
            task_queue: task_queue.queue.iter().map(TaskSaveData::from).collect(),
        }
    }
}

pub struct StationSaveData {
    position: Vec2,
}

pub struct AsteroidSaveData {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    angular_velocity: f32,
}

#[derive(Default)]
pub struct SectorSaveData {
    coordinate: Hex,
    ships: Vec<ShipSaveData>,
    stations: Vec<StationSaveData>,
    asteroids: Vec<AsteroidSaveData>,
}

pub struct GateSaveData {
    from_sector: Hex,
    from_position: Vec2,
    to_sector: Hex,
    to_position: Vec2,
}

pub struct UniverseSaveData {
    sector_save_data: SectorSaveData,
}

pub fn save(sectors: Query<&Sector>, ships: Query<(&Transform, &TaskQueue, &ShipVelocity)>) {
    sectors.par_iter().for_each(|sector| {
        let mut result = SectorSaveData::default();
        for x in &sector.ships {
            let ship = ships.get(x.into()).unwrap();
            result.ships.push(ShipSaveData::from(ship))
        }
    });
}
