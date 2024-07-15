use crate::components::{Sector, Station};
use crate::game_data::ItemId;
use crate::persistence::persistent_entity_id::{
    PersistentAsteroidId, PersistentEntityId, PersistentGateId,
};
use crate::persistence::AllEntityIdMaps;
use crate::physics::ShipVelocity;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};
use crate::utils::{AsteroidEntityWithTimestamp, ExchangeWareData};
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
    pub fn from(task: &TaskInsideQueue, all_entity_id_maps: &AllEntityIdMaps) -> Self {
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
                enter_gate: all_entity_id_maps.gates.entity_to_id()[enter_gate],
                exit_sector: all_entity_id_maps.sectors.entity_to_id()[exit_sector],
            },
            TaskInsideQueue::MineAsteroid { target, reserved } => Self::MineAsteroid {
                target: all_entity_id_maps.asteroids.entity_to_id()[target],
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
        all_entity_id_maps: &AllEntityIdMaps,
    ) -> Self {
        Self {
            position: transform.translation.truncate(),
            forward_velocity: velocity.forward,
            rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
            angular_velocity: velocity.angular,
            task_queue: task_queue
                .queue
                .iter()
                .map(|x| TaskSaveData::from(x, all_entity_id_maps))
                .collect(),
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

impl AsteroidSaveData {
    pub fn from(asteroid: AsteroidEntityWithTimestamp) {}
}

#[derive(Default)]
pub struct SectorSaveData {
    coordinate: Hex,
    live_asteroids: Vec<AsteroidSaveData>,
    respawning_asteroids: Vec<AsteroidSaveData>,
}

impl SectorSaveData {
    pub fn from(sector: &Sector) -> Self {
        todo!();
        let live_asteroids = sector.asteroids.iter().map(|x| {});

        Self {
            coordinate: sector.coordinate,
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        }
    }
}

pub struct GateSaveData {
    from_sector: Hex,
    from_position: Vec2,
    to_sector: Hex,
    to_position: Vec2,
}

pub struct UniverseSaveData {
    //sectors: Vec<SectorSaveData>,
    ships: Vec<ShipSaveData>,
}

pub fn save(
    sectors: Query<&Sector>,
    ships: Query<(&Transform, &TaskQueue, &ShipVelocity)>,
    all_entity_id_maps: AllEntityIdMaps,
) {
    // let sector_save_data = sectors
    //     .iter()
    //     .map(|query_content| SectorSaveData::from(query_content, &all_entity_id_maps);

    let ship_save_data = ships
        .iter()
        .map(|query_content| ShipSaveData::from(query_content, &all_entity_id_maps))
        .collect();

    let result = UniverseSaveData {
        ships: ship_save_data,
    };
}
