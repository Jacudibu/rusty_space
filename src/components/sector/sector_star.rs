use crate::utils::StarEntity;
use bevy::prelude::Component;

/// Marker Component for sectors featuring stars and orbital mechanics, containing the Entity for the celestial in question.
#[derive(Component)]
pub struct SectorStarComponent {
    /// The celestial which resides within this sector.
    pub entity: StarEntity,
}
