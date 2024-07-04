use crate::components::{Asteroid, Gate, Sector, Ship, Station};
use crate::utils::entity_wrappers::typed_entity::TypedEntity;

mod asteroid_with_lifetime;
mod typed_entity;

pub type SectorEntity = TypedEntity<Sector>;
pub type GateEntity = TypedEntity<Gate>;
pub type ShipEntity = TypedEntity<Ship>;
pub type StationEntity = TypedEntity<Station>;
pub type AsteroidEntity = TypedEntity<Asteroid>;

pub use asteroid_with_lifetime::AsteroidEntityWithLifetime;
