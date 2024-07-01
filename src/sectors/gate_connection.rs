use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, CubicBezier, CubicCurve, CubicGenerator, Entity, Event, EventReader,
    GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect,
};

#[derive(Component)]
pub struct GateConnection {
    pub from: Entity,
    pub to: Entity,
    pub curve: CubicCurve<Vec2>,
    pub curve_positions: Vec<Vec2>,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GateConnectionGizmos;

#[derive(Event)]
pub struct SetupGateConnectionEvent {
    pub from: Entity,
    pub to: Entity,
}

pub fn on_setup_gate_connection(
    mut commands: Commands,
    mut events: EventReader<SetupGateConnectionEvent>,
    gates: Query<&GlobalTransform>,
) {
    for event in events.read() {
        let from = &gates.get(event.from).unwrap();
        let to = &gates.get(event.to).unwrap();
        let a = from.translation().truncate();
        let b = to.translation().truncate();
        let difference = a - b;
        let diff_rot = Vec2::new(-difference.y, difference.x) * 0.075;

        let a_curve = a - difference * 0.40 + diff_rot;
        let b_curve = b + difference * 0.40 - diff_rot;

        let curve = CubicBezier::new([[a, a_curve, b_curve, b]]).to_curve();

        commands.spawn(GateConnection {
            from: event.from,
            to: event.to,
            curve_positions: curve.iter_positions(20).collect(),
            curve,
        });
    }
}

pub fn draw_gate_connections(
    mut gizmos: Gizmos<GateConnectionGizmos>,
    gate_connections: Query<&GateConnection>,
) {
    for connection in gate_connections.iter() {
        // TODO: Only do this if any of the connected gates is visible
        gizmos.linestrip_2d(
            connection.curve_positions.iter().copied(),
            bevy::color::palettes::css::GREY,
        );
    }
}
