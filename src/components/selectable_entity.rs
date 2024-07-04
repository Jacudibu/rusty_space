use bevy::prelude::Component;

/// Marker component for anything interactable.
#[derive(Component, Eq, PartialEq)]
pub enum SelectableEntity {
    Asteroid,
    Gate,
    Ship,
    Station,
}

pub const RADIUS_CURSOR: f32 = 4.0;
const RADIUS_STATION: f32 = 16.0;
const RADIUS_GATE: f32 = 16.0;
const RADIUS_SHIP: f32 = 8.0;
const RADIUS_ASTEROID: f32 = 8.0;

impl SelectableEntity {
    pub fn radius(&self) -> f32 {
        match self {
            SelectableEntity::Asteroid => RADIUS_ASTEROID,
            SelectableEntity::Gate => RADIUS_GATE,
            SelectableEntity::Ship => RADIUS_SHIP,
            SelectableEntity::Station => RADIUS_STATION,
        }
    }
}
