use crate::game_data::ItemId;
use crate::persistence::PersistentCelestialId;
use crate::utils::CelestialMass;
use bevy::prelude::Component;

/// A celestial is a naturally occurring, permanent Entity within a sector. Planets and Stars.
#[derive(Component)]
#[component(immutable)]
pub struct Celestial {
    /// The mass of this celestial, mainly used to calculate orbit speeds.
    pub mass: CelestialMass,
    pub id: PersistentCelestialId,
}

/// Specification component for a [Celestial] Entity.
/// A sector containing a star should always be marked to with a [SectorWithStar],
/// which allows us to query for the related entity with this component here.
#[derive(Component)]
#[component(immutable)]
pub struct Star {}

/// Specification component for a [Celestial] Entity.
/// This one contains a solid surface.
#[derive(Component)]
#[component(immutable)]
pub struct Planet {}

/// Specification component for a [Celestial] Entity.
/// This one contains harvestable gases.
#[derive(Component)]
#[component(immutable)]
pub struct GasGiant {
    pub resources: Vec<ItemId>,
}
