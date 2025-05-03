use crate::utils::TypedEntity;
use bevy::prelude::Component;

/// Marker Component for Ships which are docked and thus hidden.
#[derive(Component)]
#[component(immutable)]
pub struct IsDocked {
    pub at: TypedEntity,
}

impl IsDocked {
    #[inline]
    pub fn new(at: TypedEntity) -> Self {
        Self { at }
    }
}
