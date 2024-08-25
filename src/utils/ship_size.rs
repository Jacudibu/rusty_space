use serde::Deserialize;

#[derive(Deserialize)]
pub enum ShipSize {
    S,
    M,
    L,
    XL,
}
