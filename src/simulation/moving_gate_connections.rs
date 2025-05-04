use crate::components::{Gate, GateConnection, MovingGateConnection};
use bevy::prelude::{Query, Transform, With};
use common::constants::BevyResult;

pub fn update_gate_connections(
    mut connections: Query<&mut GateConnection, With<MovingGateConnection>>,
    mut gates: Query<(&mut Gate, &Transform)>, // TODO: Use SimulationTransform once orbit velocity is slowed down
) -> BevyResult {
    for mut connection in connections.iter_mut() {
        let [(mut from_gate, from_transform), (mut to_gate, to_transform)] =
            gates.get_many_mut([connection.from.into(), connection.to.into()])?;

        let (from_curve, to_curve) = GateConnection::calculate_curves_from_global_positions(
            from_transform.translation.truncate(),
            to_transform.translation.truncate(),
        );

        connection.render_positions = GateConnection::calculate_render_positions(&from_curve);
        from_gate.transit_curve = from_curve;
        to_gate.transit_curve = to_curve;
    }

    Ok(())
}
