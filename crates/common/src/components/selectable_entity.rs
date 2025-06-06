use crate::game_data::AsteroidDataId;
use crate::session_data::ShipConfigId;
use bevy::prelude::Component;

/// Marker component for anything interactable.
/// The individual enum values might further specify the selection.
#[derive(Component, Eq, PartialEq)]
#[component(immutable)]
pub enum SelectableEntity {
    Asteroid(AsteroidDataId),
    Gate,
    Celestial,
    Ship(ShipConfigId),
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
            SelectableEntity::Celestial => RADIUS_PLANET,
            SelectableEntity::Ship(_) => RADIUS_SHIP,
            SelectableEntity::Star => RADIUS_STAR,
            SelectableEntity::Station => RADIUS_STATION,
        }
    }
}
