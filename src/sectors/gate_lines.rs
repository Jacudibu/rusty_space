use crate::sectors::gate::{AllGates, GateComponent};
use bevy::prelude::{
    CubicBezier, CubicGenerator, GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect, Res,
    ViewVisibility,
};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GateLineGizmos;

pub fn draw_gate_lines(
    mut gizmos: Gizmos<GateLineGizmos>,
    gates: Query<(&GateComponent, &GlobalTransform, &ViewVisibility)>,
    all_gates: Res<AllGates>,
) {
    for (gate, transform, _) in gates.iter().filter(|(_, _, &visibility)| true /*TODO*/) {
        let other = &all_gates[&gate.id.invert()];
        let (_, other_transform, _) = gates.get(other.entity).unwrap();

        gizmos.line_2d(
            transform.translation().truncate(),
            other_transform.translation().truncate(),
            bevy::color::palettes::css::GREY,
        );
    }
}
