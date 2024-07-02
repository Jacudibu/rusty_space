use crate::constants::{GATE_CONNECTION_LAYER, SHIP_LAYER};
use crate::sectors::{GateComponent, GateId};
use crate::utils::KeyValueResource;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, CubicBezier, CubicCurve, CubicGenerator, Entity, Event, EventReader,
    GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect, ResMut, Vec3,
};

#[derive(Component)]
pub struct GateConnectionComponent {
    pub id: GateId,
    pub render_positions: Vec<Vec3>,
}

pub struct GateConnectionData {
    pub id: GateId,
    pub entity: Entity,
    pub ship_curve: CubicCurve<Vec3>,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GateConnectionGizmos;

#[derive(Event)]
pub struct SetupGateConnectionEvent {
    pub from: Entity,
    pub to: Entity,
}

pub type AllGateConnections = KeyValueResource<GateId, GateConnectionData>;

pub fn on_setup_gate_connection(
    mut commands: Commands,
    mut events: EventReader<SetupGateConnectionEvent>,
    gates: Query<(&GlobalTransform, &GateComponent)>,
    mut all_gate_connections: ResMut<AllGateConnections>,
) {
    for event in events.read() {
        let (from_transform, from_component) = &gates.get(event.from).unwrap();
        let (to_transform, to_component) = &gates.get(event.to).unwrap();
        let a = from_transform.translation().truncate();
        let b = to_transform.translation().truncate();
        let difference = a - b;
        let diff_rot = Vec2::new(-difference.y, difference.x) * 0.075;

        let a_curve = a - difference * 0.40 + diff_rot;
        let b_curve = b + difference * 0.40 - diff_rot;

        let ship_curve = create_curve(a, a_curve, b_curve, b);
        let ship_curve_inverted = create_curve(b, b_curve, a_curve, a);

        let entity = commands
            .spawn(GateConnectionComponent {
                id: from_component.id,
                render_positions: ship_curve
                    .iter_positions(20)
                    .map(|x| x.truncate().extend(GATE_CONNECTION_LAYER))
                    .collect(),
            })
            .id();

        all_gate_connections.insert(
            from_component.id,
            GateConnectionData {
                id: from_component.id,
                entity,
                ship_curve,
            },
        );
        all_gate_connections.insert(
            to_component.id,
            GateConnectionData {
                id: to_component.id,
                entity,
                ship_curve: ship_curve_inverted,
            },
        );
    }
}

fn create_curve(a: Vec2, a_curve: Vec2, b_curve: Vec2, b: Vec2) -> CubicCurve<Vec3> {
    CubicBezier::new([[
        a.extend(SHIP_LAYER),
        a_curve.extend(SHIP_LAYER),
        b_curve.extend(SHIP_LAYER),
        b.extend(SHIP_LAYER),
    ]])
    .to_curve()
}

pub fn draw_gate_connections(
    mut gizmos: Gizmos<GateConnectionGizmos>,
    gate_connections: Query<&GateConnectionComponent>,
) {
    for connection in gate_connections.iter() {
        // TODO: Only do this if the connection is visible
        gizmos.linestrip(
            connection.render_positions.iter().copied(),
            bevy::color::palettes::css::GREY,
        );
    }
}
