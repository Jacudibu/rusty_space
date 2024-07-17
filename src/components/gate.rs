use crate::persistence::{ComponentWithPersistentId, PersistentGateId, TypedPersistentEntityId};
use bevy::prelude::{Component, CubicCurve, Vec3};

/// A Gate which can be used by ships in order to transition in between sectors.
#[derive(Component)]
pub struct Gate {
    id: PersistentGateId,
    /// The curve ships will use for their sector transition after entering this gate.
    pub transit_curve: CubicCurve<Vec3>,
}

impl ComponentWithPersistentId<Gate> for Gate {
    fn id(&self) -> TypedPersistentEntityId<Gate> {
        self.id
    }
}

impl Gate {
    pub fn new(id: PersistentGateId, transit_curve: CubicCurve<Vec3>) -> Self {
        Self { id, transit_curve }
    }
}
