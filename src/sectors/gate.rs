use crate::sectors::typed_entity::TypedEntity;
use bevy::prelude::{Component, CubicCurve, Vec3};

pub type GateEntity = TypedEntity<Gate>;

#[derive(Component)]
pub struct Gate {
    pub transit_curve: CubicCurve<Vec3>,
}
