use crate::components::celestials::Celestial;
use crate::components::{Asteroid, ConstructionSite, Gate, Sector, Ship, Station};
use crate::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;

mod asteroid_with_lifetime;
mod typed_entity;
pub mod typed_entity_wrapper;

pub use typed_entity::TypedEntity;

pub type SectorEntity = TypedEntityWrapper<Sector>;
pub type GateEntity = TypedEntityWrapper<Gate>;
pub type CelestialEntity = TypedEntityWrapper<Celestial>;
pub type ShipEntity = TypedEntityWrapper<Ship>;
pub type StationEntity = TypedEntityWrapper<Station>;
pub type ConstructionSiteEntity = TypedEntityWrapper<ConstructionSite>;
pub type AsteroidEntity = TypedEntityWrapper<Asteroid>;

pub use asteroid_with_lifetime::AsteroidEntityWithTimestamp;
