use crate::components::{Gate, Sector};
use crate::utils::entity_wrappers::typed_entity::TypedEntity;

mod typed_entity;

pub type SectorEntity = TypedEntity<Sector>;
pub type GateEntity = TypedEntity<Gate>;
