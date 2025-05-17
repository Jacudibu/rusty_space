use bevy::app::App;
use bevy::prelude::{
    AppGizmoBuilder, GizmoConfigGroup, Gizmos, IntoScheduleConfigs, Plugin, Query, Reflect,
    Transform, Update, With, in_state,
};
use common::components::{Gate, GateConnection, MovingGateConnection};
use common::constants::BevyResult;
use common::states::SimulationState;

pub struct GateGizmoPlugin;
impl Plugin for GateGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<GateConnectionGizmos>();
        app.add_systems(Update, draw_gate_connections);
        app.add_systems(
            Update, // TODO: Depending on our orbit velocity, this should be running in FixedUpdate or even less often and use SimulationTransform
            update_gate_connections.run_if(in_state(SimulationState::Running)),
        );
    }
}

fn update_gate_connections(
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

#[derive(Default, Reflect, GizmoConfigGroup)]
struct GateConnectionGizmos;

fn draw_gate_connections(
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
