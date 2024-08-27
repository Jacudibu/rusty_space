use leafwing_manifest::identifier::Id;

mod ship_weapon;
mod ship_weapon_manifest;

pub type ShipWeaponId = Id<ShipWeaponData>;

pub use {ship_weapon::ShipWeaponData, ship_weapon_manifest::ShipWeaponManifest};

const MOCK_SHIP_WEAPON_ORE_MINING_LASER_NAME: &str = "Mining Laser";
pub const MOCK_SHIP_WEAPON_ORE_MINING_LASER_ID: ShipWeaponId =
    ShipWeaponId::from_name(MOCK_SHIP_WEAPON_ORE_MINING_LASER_NAME);

const MOCK_SHIP_WEAPON_GAS_COLLECTOR_NAME: &str = "Gas Collector";
pub const MOCK_SHIP_WEAPON_GAS_COLLECTOR_ID: ShipWeaponId =
    ShipWeaponId::from_name(MOCK_SHIP_WEAPON_GAS_COLLECTOR_NAME);
