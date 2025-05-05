use bevy::prelude::{PartialReflect, Resource};
use hexx::Hex;
use rand::SeedableRng;
use rand::prelude::StdRng;

/// The seed used for all random generation.
#[derive(Resource)]
pub struct UniverseSeed {
    seed: u64,
}

impl UniverseSeed {
    pub const fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    pub fn for_sector(&self, coordinates: Hex, kind: &str) -> StdRng {
        let string = format!("{}:{kind}:{}.{}", self.seed, coordinates.x, coordinates.y);
        let seed = string.reflect_hash().unwrap();
        StdRng::seed_from_u64(seed)
    }
}
