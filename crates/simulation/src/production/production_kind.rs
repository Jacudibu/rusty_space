use common::game_data::{ProductionModuleId, ShipyardModuleId};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ProductionKind {
    Item(ProductionModuleId),
    Shipyard(ShipyardModuleId),
}
