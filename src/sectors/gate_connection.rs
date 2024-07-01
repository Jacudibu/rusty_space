use crate::sectors::{GateComponent, GateId};
use crate::utils::KeyValueResource;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, CubicBezier, CubicCurve, CubicGenerator, Entity, Event, EventReader,
    GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect, ResMut,
};

#[derive(Component)]
pub struct GateConnection {
    pub id: GateId,
    pub from: Entity,
    pub to: Entity,
    pub curve: CubicCurve<Vec2>,
    pub curve_positions: Vec<Vec2>,
}

pub enum GateConnectionDirection {
    Regular(Entity),
    Inverted(Entity),
}

impl GateConnectionDirection {
    pub fn evaluate(&self, connection: &GateConnection, t: f32) -> Vec2 {
        match self {
            GateConnectionDirection::Regular(_) => connection.curve.position(t),
            GateConnectionDirection::Inverted(_) => connection.curve.position(1.0 - t),
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

        let curve = CubicBezier::new([[a, a_curve, b_curve, b]]).to_curve();

        let entity = commands
            .spawn(GateConnection {
                id: from_id.id,
                from: event.from,
                to: event.to,
                curve_positions: curve.iter_positions(20).collect(),
                curve,
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
        // TODO: Only do this if any of the connected gates is visible
        gizmos.linestrip_2d(
            connection.curve_positions.iter().copied(),
            bevy::color::palettes::css::GREY,
        );
    }
}
