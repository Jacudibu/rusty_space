use crate::utils::SolarMass;
use bevy::prelude::Component;

/// Marker Component for sectors featuring stars and orbital mechanics.
#[derive(Component)]
pub struct SectorStarComponent {
    pub mass: SolarMass,
}
