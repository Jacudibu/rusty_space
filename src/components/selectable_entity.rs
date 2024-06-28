use bevy::prelude::Component;

/// Marker component for anything interactable.
#[derive(Component, Eq, PartialEq)]
pub enum SelectableEntity {
    Station,
    Ship,
}

pub const RADIUS_CURSOR: f32 = 4.0;
const RADIUS_STATION: f32 = 16.0;
const RADIUS_SHIP: f32 = 8.0;

impl SelectableEntity {
    pub fn radius(&self) -> f32 {
        match self {
            SelectableEntity::Station => RADIUS_STATION,
            SelectableEntity::Ship => RADIUS_SHIP,
        }
    }
}
