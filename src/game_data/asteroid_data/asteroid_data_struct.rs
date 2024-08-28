use crate::game_data::ItemId;
use bevy::asset::Handle;
use bevy::prelude::Image;
use std::ops::Range;

pub struct AsteroidData {
    pub material: ItemId,
    pub amount: Range<u32>,
    pub sprite: Handle<Image>,
}
