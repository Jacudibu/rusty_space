use leafwing_manifest::identifier::Id;

mod ship_weapon;
mod ship_weapon_manifest;

pub type ShipWeaponId = Id<ShipWeaponData>;

use crate::create_id_constants;
pub use {ship_weapon::ShipWeaponData, ship_weapon_manifest::ShipWeaponManifest};

create_id_constants!(
    ShipWeaponId,
    CONSTRUCTION_TOOL,
    ORE_MINING_LASER,
    GAS_COLLECTOR
);
