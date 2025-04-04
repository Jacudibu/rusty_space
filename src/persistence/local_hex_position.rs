use crate::components::{InSector, SectorComponent};
use crate::persistence::SectorIdMap;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::SectorPosition;
use bevy::math::Vec2;
use bevy::prelude::Query;
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
    pub fn from(sector: &SectorComponent, transform: &SimulationTransform) -> Self {
        Self {
            sector: sector.coordinate,
            position: transform.translation - sector.world_pos,
        }
    }

    #[inline]
    pub fn from_in_sector(
        in_sector: &InSector,
        transform: &SimulationTransform,
        sectors: &Query<&SectorComponent>,
    ) -> Self {
        let sector = sectors.get(in_sector.sector.into()).unwrap();
        Self {
            sector: sector.coordinate,
            position: transform.translation - sector.world_pos,
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
