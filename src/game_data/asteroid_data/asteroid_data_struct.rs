use crate::game_data::ItemId;
use bevy::asset::Handle;
use bevy::color::Color;
use bevy::prelude::Image;
use std::ops::Range;

/// Mainly used during universe generation to spawn asteroids.
pub struct AsteroidData {
    pub name: String,
    pub material: ItemId,
    pub amount: Range<u32>,
    pub sprite: Handle<Image>,
    pub sprite_color: Color,
}
