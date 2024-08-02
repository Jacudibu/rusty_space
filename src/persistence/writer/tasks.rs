use crate::persistence::data::v1::*;
use crate::persistence::AllEntityIdMaps;
use crate::simulation::ship_ai::TaskInsideQueue;
use crate::utils::ExchangeWareData;

impl TaskSaveData {
    pub fn from(task: &TaskInsideQueue, all_entity_id_maps: &AllEntityIdMaps) -> Self {
        match task {
            TaskInsideQueue::ExchangeWares { target, data } => Self::ExchangeWares {
                target: all_entity_id_maps.get_typed_id_unchecked(target),
                data: data.into(),
            },
            TaskInsideQueue::MoveToEntity {
                target,
                stop_at_target,
                distance_to_target: distance,
            } => Self::MoveToEntity {
                target: all_entity_id_maps.get_typed_id_unchecked(target),
                stop_at_target: *stop_at_target,
                distance_to_target: *distance,
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
            TaskInsideQueue::HarvestGas { target } => Self::HarvestGas {
                target: all_entity_id_maps.planets.entity_to_id()[target],
            },
            TaskInsideQueue::DockAtEntity { .. } => {
                todo!()
            }
            TaskInsideQueue::AwaitingSignal => {
                todo!()
            }
            TaskInsideQueue::RequestAccess { .. } => {
                todo!()
            }
            TaskInsideQueue::Undock => {
                todo!()
            }
        }
    }
}

impl From<&ExchangeWareData> for ExchangeWareSaveData {
    fn from(value: &ExchangeWareData) -> Self {
        match value {
            ExchangeWareData::Buy(item, amount) => Self::Buy(*item, *amount),
            ExchangeWareData::Sell(item, amount) => Self::Sell(*item, *amount),
        }
    }
}
