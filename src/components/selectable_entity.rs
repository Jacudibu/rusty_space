use bevy::prelude::Component;

/// Marker component for anything interactable.
#[derive(Component)]
pub enum SelectableEntity {
    Station,
    Ship,
}
