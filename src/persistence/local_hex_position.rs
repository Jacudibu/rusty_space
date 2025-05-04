use crate::persistence::SectorIdMap;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::SectorPosition;
use bevy::math::Vec2;
use bevy::prelude::Query;
use common::components::{InSector, Sector};
use common::types::polar_coordinates::PolarCoordinates;
use hexx::Hex;
use serde::{Deserialize, Serialize};

/// Represents a persist-able local position of an object within a specific hexagon.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct LocalHexPosition {
    /// The [Hex] representing the sector of this position.
    pub sector: Hex,
    /// The local position within the sector.
    pub position: Vec2,
}

impl LocalHexPosition {
    #[inline]
    pub fn new(sector: Hex, position: Vec2) -> Self {
        Self { sector, position }
    }

    /// Creates a new instance of [LocalHexPosition] from [PolarCoordinates]
    #[inline]
    pub fn from_polar(sector: Hex, position: PolarCoordinates) -> Self {
        Self {
            sector,
            position: position.to_cartesian(),
        }
    }

    #[inline]
    pub fn from(sector: &Sector, transform: &SimulationTransform) -> Self {
        Self {
            sector: sector.coordinate,
            position: transform.translation - sector.world_pos,
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
