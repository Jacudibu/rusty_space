use crate::constants::{GATE_CONNECTION_LAYER, SHIP_LAYER};
use crate::sectors::{GateComponent, GateId};
use crate::utils::KeyValueResource;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, CubicBezier, CubicCurve, CubicGenerator, Entity, Event, EventReader,
    GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect, ResMut, Vec3,
};

#[derive(Component)]
pub struct GateConnection {
    pub id: GateId,
    pub from: Entity,
    pub to: Entity,
    pub ship_curve: CubicCurve<Vec3>,
    pub render_positions: Vec<Vec3>,
}

pub enum GateConnectionDirection {
    Regular(Entity),
    Inverted(Entity),
}

impl GateConnectionDirection {
    pub fn evaluate_ship_position(&self, connection: &GateConnection, t: f32) -> Vec3 {
        match self {
            GateConnectionDirection::Regular(_) => connection.ship_curve.position(t),
            GateConnectionDirection::Inverted(_) => connection.ship_curve.position(1.0 - t),
        }
    }

    pub fn inner(&self) -> Entity {
        match self {
            GateConnectionDirection::Regular(e) => *e,
            GateConnectionDirection::Inverted(e) => *e,
        }
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GateConnectionGizmos;

#[derive(Event)]
pub struct SetupGateConnectionEvent {
    pub from: Entity,
    pub to: Entity,
}

pub type AllGateConnections = KeyValueResource<GateId, GateConnectionDirection>;

pub fn on_setup_gate_connection(
    mut commands: Commands,
    mut events: EventReader<SetupGateConnectionEvent>,
    gates: Query<(&GlobalTransform, &GateComponent)>,
    mut all_gate_connections: ResMut<AllGateConnections>,
) {
    for event in events.read() {
        let (from_transform, from_id) = &gates.get(event.from).unwrap();
        let (to_transform, to_id) = &gates.get(event.to).unwrap();
        let a = from_transform.translation().truncate();
        let b = to_transform.translation().truncate();
        let difference = a - b;
        let diff_rot = Vec2::new(-difference.y, difference.x) * 0.075;

        let a_curve = a - difference * 0.40 + diff_rot;
        let b_curve = b + difference * 0.40 - diff_rot;

        let curve = CubicBezier::new([[
            a.extend(SHIP_LAYER),
            a_curve.extend(SHIP_LAYER),
            b_curve.extend(SHIP_LAYER),
            b.extend(SHIP_LAYER),
        ]])
        .to_curve();

        let entity = commands
            .spawn(GateConnection {
                id: from_id.id,
                from: event.from,
                to: event.to,
                render_positions: curve
                    .iter_positions(20)
                    .map(|x| x.truncate().extend(GATE_CONNECTION_LAYER))
                    .collect(),
                ship_curve: curve,
            })
            .id();

        all_gate_connections.insert(from_id.id, GateConnectionDirection::Regular(entity));
        all_gate_connections.insert(to_id.id, GateConnectionDirection::Inverted(entity));
    }
}

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
