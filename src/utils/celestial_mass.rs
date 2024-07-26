use serde::{Deserialize, Serialize};

/// Solar Mass represented as integer.
/// Stellar Mass = 0.01 Solar Masses. The smallest stars are about 0.08 Solar Masses. For anything smaller than that, use [`EarthMass`].
/// The biggest known stars are 250 Solar Masses (with a theoretical maximum of 300 at the early stage of a universe)
/// Stellar black holes which might end up getting used in our simulation usually have only 3-100 Solar Masses
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct SolarMass(u32);
impl SolarMass {
    pub fn from_solar_mass(solar_masses: u32, fraction: u8) -> Self {
        Self(solar_masses * 100 + fraction as u32)
    }

    pub fn as_solar_mass(&self) -> f32 {
        self.0 as f32 * 0.01
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

/// Earth Mass represented as integer.
/// The smallest supported value is 0.00001 Earth Masses, which is small enough to fit dwarf planets like Pluto (0.0025) and Ceres (0.00015)
/// The highest depictable mass is 42949.67295, which equals 135 jupiter masses or ~0.13 Solar Masses. Use [`SolarMass`] for anything bigger.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct EarthMass(u32);
impl EarthMass {
    pub fn from_earth_mass(earth_masses: u16, fraction: u16) -> Self {
        Self(earth_masses as u32 * 100000 + fraction as u32)
    }

    pub fn as_earth_mass(&self) -> f32 {
        self.0 as f32 * 0.00001
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}
