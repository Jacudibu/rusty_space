use bevy::prelude::Resource;
use bevy::reflect::Reflect;
use hexx::Hex;
use rand::prelude::StdRng;
use rand::SeedableRng;

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
