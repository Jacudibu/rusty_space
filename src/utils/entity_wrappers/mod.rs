use crate::components::{
    Asteroid, ConstructionSiteComponent, GateComponent, PlanetComponent, SectorComponent, Ship,
    StarComponent, StationComponent,
};
use crate::utils::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;

mod asteroid_with_lifetime;
mod typed_entity;
mod typed_entity_wrapper;

pub use typed_entity::TypedEntity;

pub type SectorEntity = TypedEntityWrapper<SectorComponent>;
pub type GateEntity = TypedEntityWrapper<GateComponent>;
pub type PlanetEntity = TypedEntityWrapper<PlanetComponent>;
pub type ShipEntity = TypedEntityWrapper<Ship>;
pub type StarEntity = TypedEntityWrapper<StarComponent>;
pub type StationEntity = TypedEntityWrapper<StationComponent>;
pub type ConstructionSiteEntity = TypedEntityWrapper<ConstructionSiteComponent>;
pub type AsteroidEntity = TypedEntityWrapper<Asteroid>;

pub use asteroid_with_lifetime::AsteroidEntityWithTimestamp;
