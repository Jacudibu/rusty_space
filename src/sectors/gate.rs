use crate::sectors::typed_entity::TypedEntity;
use crate::sectors::SectorEntity;
use bevy::prelude::{Component, CubicCurve, Vec3};

pub type GateEntity = TypedEntity<Gate>;

#[derive(Component)]
pub struct Gate {
    pub connected_sectors: GateConnectedSectors,
}

#[derive(Component)]
pub struct GateTransitCurve {
    pub transit_curve: CubicCurve<Vec3>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GateConnectedSectors {
    pub from: SectorEntity,
    pub to: SectorEntity,
}

impl GateConnectedSectors {
    /// Returns the ID for the connected gate.
    pub fn invert(&self) -> Self {
        GateConnectedSectors {
            from: self.to,
            to: self.from,
        }
    }
}
