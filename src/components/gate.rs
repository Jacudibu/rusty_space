use bevy::prelude::{Component, CubicCurve, Vec3};

/// A Gate which can be used by ships in order to transition in between sectors.
#[derive(Component)]
pub struct Gate {
    /// The curve ships will use for their sector transition after entering this gate.
    pub transit_curve: CubicCurve<Vec3>,
}
