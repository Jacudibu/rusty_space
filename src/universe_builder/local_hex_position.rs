use crate::persistence::SectorIdMap;
use crate::utils::SectorPosition;
use bevy::math::Vec2;
use hexx::Hex;

pub struct LocalHexPosition {
    pub sector: Hex,
    pub position: Vec2,
}

impl LocalHexPosition {
    pub fn new(sector: Hex, position: Vec2) -> Self {
        Self { sector, position }
    }

    pub fn to_sector_position(&self, sector_id_map_entity_map: &SectorIdMap) -> SectorPosition {
        SectorPosition {
            sector: sector_id_map_entity_map.id_to_entity()[&self.sector],
            local_position: self.position,
        }
    }
}
