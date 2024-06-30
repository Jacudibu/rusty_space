use crate::game_data::ShipyardModuleId;
use crate::session_data::ShipConfigId;
use crate::utils::Milliseconds;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct ShipyardComponent {
    pub modules: HashMap<ShipyardModuleId, ShipyardModule>,
    pub queue: Vec<ShipConfigId>,
}

pub struct ShipyardModule {
    pub amount: u32,
    pub active: Vec<OngoingShipConstructionOrder>,
}

pub struct OngoingShipConstructionOrder {
    pub ship_config: ShipConfigId,
    pub finished_at: Milliseconds,
}
