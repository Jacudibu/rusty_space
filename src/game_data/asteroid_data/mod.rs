use crate::game_data::asteroid_data::asteroid_data_struct::AsteroidData;
use leafwing_manifest::identifier::Id;

mod asteroid_data_struct;
mod asteroid_manifest;
mod raw_asteroid_data;
mod raw_asteroid_manifest;

pub type AsteroidDataId = Id<AsteroidData>;
