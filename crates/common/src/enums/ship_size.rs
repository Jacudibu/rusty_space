use serde::Deserialize;

/// Ships are roughly classified by the size of their chassis.
/// This primarily limits docking capabilities: An XL-Class Ship won't be able to dock at an M-Class ship.
#[derive(Deserialize)]
pub enum ShipSize {
    S,
    M,
    L,
    XL,
}
