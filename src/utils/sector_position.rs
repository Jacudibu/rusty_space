use crate::sectors::SectorId;
use bevy::math::Vec2;

pub struct SectorPosition {
    pub sector: SectorId,
    pub local_position: Vec2,
}
