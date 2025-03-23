use crate::game_data::ItemId;
use crate::persistence::PersistentPlanetId;
use crate::utils::EarthMass;
use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct SectorPlanetSaveData {
    pub id: PersistentPlanetId,
    pub kind: PlanetKindSaveData,
    pub name: String,
    pub mass: EarthMass,
    pub local_position: Vec2,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum PlanetKindSaveData {
    Terrestrial,
    GasGiant { resources: Vec<ItemId> },
}
