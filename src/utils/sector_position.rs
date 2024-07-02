use bevy::math::Vec2;
use hexx::Hex;

pub struct SectorPosition {
    pub sector: Hex,
    pub local_position: Vec2,
}
