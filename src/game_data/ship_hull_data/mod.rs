use leafwing_manifest::identifier::Id;

mod ship_hull;
mod ship_hull_manifest;

pub use {ship_hull::ShipHullData, ship_hull_manifest::ShipHullManifest};

pub type ShipHullId = Id<ShipHullData>;

const MOCK_SHIP_A_NAME: &str = "a";
pub const MOCK_SHIP_A_ID: ShipHullId = ShipHullId::from_name(MOCK_SHIP_A_NAME);
