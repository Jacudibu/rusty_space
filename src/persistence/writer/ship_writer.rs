use crate::components::{InSector, Inventory, Sector, Ship};
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{AllEntityIdMaps, ComponentWithPersistentId};
use crate::simulation::physics::ShipVelocity;
use crate::simulation::ship_ai::{AutoMineBehavior, AutoTradeBehavior, TaskQueue};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use bevy::core::Name;
use bevy::prelude::Query;

impl ShipSaveData {
    pub fn from(
        (ship, name, in_sector, transform, task_queue, velocity, inventory, auto_trade, auto_mine): (
            &Ship,
            &Name,
            &InSector,
            &SimulationTransform,
            &TaskQueue,
            &ShipVelocity,
            &Inventory,
            Option<&AutoTradeBehavior>,
            Option<&AutoMineBehavior>,
        ),
        sectors: &Query<&Sector>,
        all_entity_id_maps: &AllEntityIdMaps,
    ) -> Self {
        Self {
            id: ship.id(),
            config_id: ship.config_id(),
            name: name.to_string(),
            position: LocalHexPosition::from_in_sector(in_sector, transform, sectors),
            forward_velocity: velocity.forward,
            rotation_degrees: transform.rotation.as_degrees(),
            angular_velocity: velocity.angular,
            behavior: ShipBehaviorSaveData::from(auto_trade, auto_mine),
            task_queue: task_queue
                .queue
                .iter()
                .map(|x| TaskSaveData::from(x, all_entity_id_maps))
                .collect(),
            inventory: InventorySaveData::from(inventory),
        }
    }
}

impl ShipBehaviorSaveData {
    pub fn from(
        auto_trade: Option<&AutoTradeBehavior>,
        auto_mine: Option<&AutoMineBehavior>,
    ) -> Self {
        if let Some(auto_trade) = auto_trade {
            return ShipBehaviorSaveData::AutoTrade {
                next_idle_update: auto_trade.next_idle_update,
            };
        }
        if let Some(auto_mine) = auto_mine {
            return ShipBehaviorSaveData::AutoTrade {
                next_idle_update: auto_mine.next_idle_update,
            };
        }

        panic!("Ships should always have one behavior!")
    }
}
