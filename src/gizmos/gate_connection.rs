use bevy::prelude::{GizmoConfigGroup, Gizmos, Query, Reflect};

use crate::components::GateConnection;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GateConnectionGizmos;

pub fn draw_gate_connections(
    mut gizmos: Gizmos<GateConnectionGizmos>,
    gate_connections: Query<&GateConnection>,
) {
    for connection in gate_connections.iter() {
        // TODO: Only do this if the connection is visible
        gizmos.linestrip(
            connection.render_positions.iter().copied(),
            bevy::color::palettes::css::GREY,
        );
    }
}
