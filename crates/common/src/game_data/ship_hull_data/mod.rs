use leafwing_manifest::identifier::Id;

mod raw_ship_hull;
mod raw_ship_hull_manifest;
mod ship_hull;
mod ship_hull_manifest;

use crate::create_id_constants;
pub use {ship_hull::ShipHullData, ship_hull_manifest::ShipHullManifest};

pub type ShipHullId = Id<ShipHullData>;

create_id_constants!(ShipHullId, SHIP_HULL_TRANSPORT);
create_id_constants!(ShipHullId, SHIP_HULL_MINER);
