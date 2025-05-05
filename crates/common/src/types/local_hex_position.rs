use crate::components::{InSector, Sector};
use crate::simulation_transform::SimulationTransform;
use crate::types::entity_id_map::SectorIdMap;
use crate::types::polar_coordinates::PolarCoordinates;
use crate::types::sector_position::SectorPosition;
use bevy::math::Vec2;
use bevy::prelude::Query;
use hexx::Hex;
use serde::{Deserialize, Serialize};

/// Represents a persist-able global position through the hex coordinates of the sector and a local position with it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocalHexPosition {
    /// The [Hex] representing the sector of this position.
    pub sector: Hex,
    /// The local position within the sector.
    pub local_position: Vec2,
}

impl LocalHexPosition {
    #[inline]
    pub fn new(sector: Hex, local_position: Vec2) -> Self {
        Self {
            sector,
            local_position,
        }
    }

    /// Creates a new instance of [LocalHexPosition] from [PolarCoordinates]
    #[inline]
    pub fn from_polar(sector: Hex, position: PolarCoordinates) -> Self {
        Self {
            sector,
            local_position: position.to_cartesian(),
        }
    }

    #[inline]
    pub fn from(sector: &Sector, transform: &SimulationTransform) -> Self {
        Self {
            sector: sector.coordinate,
            local_position: transform.translation - sector.world_pos,
        }
    }

    #[inline]
    pub fn from_in_sector(
        in_sector: &InSector,
        transform: &SimulationTransform,
        sectors: &Query<&Sector>,
    ) -> Self {
        let sector = sectors.get(in_sector.sector.into()).unwrap();
        Self {
            sector: sector.coordinate,
            local_position: transform.translation - sector.world_pos,
        }
    }

    /// Converts this [LocalHexPosition] to a [SectorPosition]
    #[inline]
    pub fn to_sector_position(&self, sector_id_map: &SectorIdMap) -> SectorPosition {
        SectorPosition {
            sector: sector_id_map.id_to_entity()[&self.sector],
            local_position: self.local_position,
        }
    }
}
