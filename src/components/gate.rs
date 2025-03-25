use crate::persistence::{ComponentWithPersistentId, PersistentGateId, TypedPersistentEntityId};
use bevy::prelude::{Component, CubicCurve, Vec2};

/// A Gate which can be used by ships in order to transition in between sectors.
#[derive(Component)]
pub struct GateComponent {
    id: PersistentGateId,
    /// The curve ships will use for their sector transition after entering this gate.
    pub transit_curve: CubicCurve<Vec2>,
}

impl ComponentWithPersistentId<GateComponent> for GateComponent {
    #[inline]
    fn id(&self) -> TypedPersistentEntityId<GateComponent> {
        self.id
    }
}

impl GateComponent {
    #[inline]
    pub fn new(id: PersistentGateId, transit_curve: CubicCurve<Vec2>) -> Self {
        Self { id, transit_curve }
    }
}
