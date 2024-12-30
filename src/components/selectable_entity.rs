use crate::game_data::AsteroidDataId;
use bevy::prelude::Component;

/// Marker component for anything interactable.
#[derive(Component, Eq, PartialEq)]
pub enum SelectableEntity {
    Asteroid(AsteroidDataId),
    Gate,
    Planet,
    Ship,
    Star,
    Station,
}

pub const RADIUS_CURSOR: f32 = 4.0;
const RADIUS_STATION: f32 = 16.0;
const RADIUS_GATE: f32 = 16.0;
const RADIUS_PLANET: f32 = 16.0;
const RADIUS_SHIP: f32 = 8.0;
const RADIUS_STAR: f32 = 16.0;
const RADIUS_ASTEROID: f32 = 8.0;

impl SelectableEntity {
    pub fn radius(&self) -> f32 {
        match self {
            SelectableEntity::Asteroid(_) => RADIUS_ASTEROID,
            SelectableEntity::Gate => RADIUS_GATE,
            SelectableEntity::Planet => RADIUS_PLANET,
            SelectableEntity::Ship => RADIUS_SHIP,
            SelectableEntity::Star => RADIUS_STAR,
            SelectableEntity::Station => RADIUS_STATION,
        }
    }
}
