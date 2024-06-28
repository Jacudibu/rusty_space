use bevy::prelude::Component;

/// Marker component for anything interactable.
#[derive(Component, Eq, PartialEq)]
pub enum SelectableEntity {
    Station,
    Ship,
}
