use crate::utils::SectorEntity;
use bevy::math::Vec2;

pub struct SectorPosition {
    pub sector: SectorEntity,
    pub local_position: Vec2,
}
