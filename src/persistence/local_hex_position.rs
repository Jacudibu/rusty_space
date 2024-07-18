use crate::components::{InSector, Sector};
use crate::persistence::SectorIdMap;
use crate::utils::SectorPosition;
use bevy::math::Vec2;
use bevy::prelude::{Query, Transform};
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
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
    pub fn from(sector: &Sector, transform: &Transform) -> Self {
        Self {
            sector: sector.coordinate,
            position: transform.translation.truncate() - sector.world_pos,
        }
    }

    #[inline]
    pub fn from_in_sector(
        in_sector: &InSector,
        transform: &Transform,
        sectors: &Query<&Sector>,
    ) -> Self {
        let sector = sectors.get(in_sector.sector.into()).unwrap();
        Self {
            sector: sector.coordinate,
            position: transform.translation.truncate() - sector.world_pos,
        }
    }

    #[inline]
    pub fn to_sector_position(&self, sector_id_map: &SectorIdMap) -> SectorPosition {
        SectorPosition {
            sector: sector_id_map.id_to_entity()[&self.sector],
            local_position: self.position,
        }
    }
}
