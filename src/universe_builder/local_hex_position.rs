use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
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

    pub fn to_sector_position(
        &self,
        hex_to_sector_entity_map: &HexToSectorEntityMap,
    ) -> SectorPosition {
        SectorPosition {
            sector: hex_to_sector_entity_map.map[&self.sector],
            local_position: self.position,
        }
    }
}
