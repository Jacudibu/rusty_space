use crate::components::{GateComponent, GateConnectionComponent, MovingGateConnection};
use bevy::prelude::{Query, Transform, With};

pub fn update_gate_connections(
    mut connections: Query<&mut GateConnectionComponent, With<MovingGateConnection>>,
    mut gates: Query<(&mut GateComponent, &Transform)>, // TODO: Use SimulationTransform once orbit velocity is slowed down
) {
    for mut connection in connections.iter_mut() {
        let [(mut from_gate, from_transform), (mut to_gate, to_transform)] =
            gates.many_mut([connection.from.into(), connection.to.into()]);

        let (from_curve, to_curve) =
            GateConnectionComponent::calculate_curves_from_global_positions(
                from_transform.translation.truncate(),
                to_transform.translation.truncate(),
            );

        connection.render_positions =
            GateConnectionComponent::calculate_render_positions(&from_curve);
        from_gate.transit_curve = from_curve;
        to_gate.transit_curve = to_curve;
    }
}
