use crate::utils::StarEntity;
use bevy::prelude::Component;

/// Marker Component for sectors featuring stars and orbital mechanics.
#[derive(Component)]
pub struct SectorStarComponent {
    pub entity: StarEntity,
}
