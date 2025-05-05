use crate::types::entity_wrappers::SectorEntity;
use bevy::math::Vec2;

/// Defines a global position through the sector and the local position within it.
#[derive(Copy, Clone)]
pub struct SectorPosition {
    /// The sector of this position
    pub sector: SectorEntity,
    /// The local position within the sector
    pub local_position: Vec2,
}
