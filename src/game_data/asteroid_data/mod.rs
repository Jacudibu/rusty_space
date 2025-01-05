use crate::create_id_constants;
use leafwing_manifest::identifier::Id;

mod asteroid_data_struct;
mod asteroid_manifest;
mod raw_asteroid_data;
mod raw_asteroid_manifest;

pub use {asteroid_data_struct::AsteroidData, asteroid_manifest::AsteroidManifest};

pub type AsteroidDataId = Id<AsteroidData>;

create_id_constants!(AsteroidDataId, IRON_ASTEROID);
create_id_constants!(AsteroidDataId, CRYSTAL_ASTEROID);
