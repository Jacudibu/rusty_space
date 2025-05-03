use crate::utils::SolarMass;
use bevy::prelude::Component;
use hexx::Hex;

/// Component for Celestials.
/// A sector containing a star should always be marked to with a [SectorStarComponent],
/// which allows us to query for the related entity with this component here.
#[derive(Component)]
#[component(immutable)]
pub struct Star {
    /// The persistent ID of this Star. // TODO: Probably shouldn't just be a hex
    pub id: Hex,
    /// The mass of this star, mainly used to calculate orbit speeds. See [SolarMass] for more information.
    pub mass: SolarMass,
}

impl Star {
    #[inline]
    pub fn new(id: Hex, mass: SolarMass) -> Self {
        Self { id, mass }
    }
}
