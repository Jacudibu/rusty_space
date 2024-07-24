use crate::components::{Asteroid, Gate, Planet, Sector, Ship, Star, Station};
use crate::utils::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;
mod asteroid_with_lifetime;
mod typed_entity;
mod typed_entity_wrapper;

pub use typed_entity::TypedEntity;

pub type SectorEntity = TypedEntityWrapper<Sector>;
pub type GateEntity = TypedEntityWrapper<Gate>;
pub type PlanetEntity = TypedEntityWrapper<Planet>;
pub type ShipEntity = TypedEntityWrapper<Ship>;
pub type StarEntity = TypedEntityWrapper<Star>;
pub type StationEntity = TypedEntityWrapper<Station>;
pub type AsteroidEntity = TypedEntityWrapper<Asteroid>;

pub use asteroid_with_lifetime::AsteroidEntityWithTimestamp;
