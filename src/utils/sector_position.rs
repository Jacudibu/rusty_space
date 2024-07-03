use bevy::math::Vec2;
use bevy::prelude::Entity;

pub struct SectorPosition {
    pub sector: Entity,
    pub local_position: Vec2,
}
