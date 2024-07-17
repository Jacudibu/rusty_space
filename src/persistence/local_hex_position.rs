use crate::components::Sector;
use crate::persistence::SectorIdMap;
use crate::utils::SectorPosition;
use bevy::math::Vec2;
use bevy::prelude::Transform;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalHexPosition {
    pub sector: Hex,
    pub position: Vec2,
}

impl LocalHexPosition {
    #[inline]
    pub fn new(sector: Hex, position: Vec2) -> Self {
        Self { sector, position }
    }

    #[inline]
    pub fn from(sector: &Sector, gate_transform: &Transform) -> Self {
        Self {
            sector: sector.coordinate,
            position: gate_transform.translation.truncate() - sector.world_pos,
        }
    }

    #[inline]
    pub fn to_sector_position(&self, sector_id_map_entity_map: &SectorIdMap) -> SectorPosition {
        SectorPosition {
            sector: sector_id_map_entity_map.id_to_entity()[&self.sector],
            local_position: self.position,
        }
    }
}
