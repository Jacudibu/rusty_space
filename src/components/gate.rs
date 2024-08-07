use crate::persistence::{ComponentWithPersistentId, PersistentGateId, TypedPersistentEntityId};
use bevy::prelude::{Component, CubicCurve, Vec2};

/// A Gate which can be used by ships in order to transition in between sectors.
#[derive(Component)]
pub struct Gate {
    id: PersistentGateId,
    /// The curve ships will use for their sector transition after entering this gate.
    pub transit_curve: CubicCurve<Vec2>,
}

impl ComponentWithPersistentId<Gate> for Gate {
    #[inline]
    fn id(&self) -> TypedPersistentEntityId<Gate> {
        self.id
    }
}

impl Gate {
    #[inline]
    pub fn new(id: PersistentGateId, transit_curve: CubicCurve<Vec2>) -> Self {
        Self { id, transit_curve }
    }
}
