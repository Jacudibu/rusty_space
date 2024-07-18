use bevy::core::Name;
use bevy::prelude::{
    Commands, CubicBezier, CubicCurve, CubicGenerator, Query, SpriteBundle, Transform, Vec2, Vec3,
};

use crate::components::{Gate, GateConnectionComponent, Sector, SelectableEntity};
use crate::constants::{GATE_CONNECTION_LAYER, SHIP_LAYER};
use crate::persistence::{GateIdMap, PersistentGateId};
use crate::utils::GateEntity;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};

pub fn spawn_gate_pair(
    commands: &mut Commands,
    gate_id_map: &mut GateIdMap,
    sector_query: &mut Query<&mut Sector>,
    sprites: &SpriteHandles,
    from_pos: SectorPosition,
    to_pos: SectorPosition,
) {
    let from_id = PersistentGateId::next();
    let to_id = PersistentGateId::next();

    spawn_gate_pair_with_ids(
        commands,
        gate_id_map,
        sector_query,
        sprites,
        from_id,
        from_pos,
        to_id,
        to_pos,
    )
}

pub fn spawn_gate_pair_with_ids(
    commands: &mut Commands,
    gate_id_map: &mut GateIdMap,
    sector_query: &mut Query<&mut Sector>,
    sprites: &SpriteHandles,
    from_id: PersistentGateId,
    from_pos: SectorPosition,
    to_id: PersistentGateId,
    to_pos: SectorPosition,
) {
    let [mut from_sector, mut to_sector] = sector_query
        .get_many_mut([from_pos.sector.into(), to_pos.sector.into()])
        .unwrap();

    let (from_curve, to_curve) = setup_connection(
        commands,
        &from_sector,
        from_pos.local_position,
        &to_sector,
        to_pos.local_position,
    );

    let from_gate = spawn_gate(
        commands,
        from_id,
        gate_id_map,
        sprites,
        &from_pos,
        &mut from_sector,
        &to_sector,
        from_curve,
    );
    let to_gate = spawn_gate(
        commands,
        to_id,
        gate_id_map,
        sprites,
        &to_pos,
        &mut to_sector,
        &from_sector,
        to_curve,
    );

    from_sector.add_gate(commands, from_pos.sector, from_gate, to_pos.sector, to_gate);
    to_sector.add_gate(commands, to_pos.sector, to_gate, from_pos.sector, from_gate);
}

fn setup_connection(
    commands: &mut Commands,
    from_sector: &Sector,
    from_pos: Vec2,
    to_sector: &Sector,
    to_pos: Vec2,
) -> (CubicCurve<Vec3>, CubicCurve<Vec3>) {
    let a = from_sector.world_pos + from_pos;
    let b = to_sector.world_pos + to_pos;
    let difference = a - b;
    let diff_rot = Vec2::new(-difference.y, difference.x) * 0.075;

    let a_curve = a - difference * 0.40 + diff_rot;
    let b_curve = b + difference * 0.40 - diff_rot;

    let ship_curve = create_curve(a, a_curve, b_curve, b);
    let ship_curve_inverted = create_curve(b, b_curve, a_curve, a);

    commands.spawn(GateConnectionComponent {
        render_positions: ship_curve
            .iter_positions(20)
            .map(|x| x.truncate().extend(GATE_CONNECTION_LAYER))
            .collect(),
    });

    (ship_curve, ship_curve_inverted)
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

fn spawn_gate(
    commands: &mut Commands,
    id: PersistentGateId,
    gate_id_map: &mut GateIdMap,
    sprites: &SpriteHandles,
    pos: &SectorPosition,
    from: &mut Sector,
    to: &Sector,
    ship_curve: CubicCurve<Vec3>,
) -> GateEntity {
    let position = from.world_pos + pos.local_position;
    let entity = commands
        .spawn((
            Name::new(format!(
                "Gate [{},{}] -> [{},{}]",
                from.coordinate.x, from.coordinate.y, to.coordinate.x, to.coordinate.y
            )),
            Gate::new(id, ship_curve),
            SelectableEntity::Gate,
            SpriteBundle {
                transform: Transform::from_translation(position.extend(constants::GATE_LAYER)),
                texture: sprites.gate.clone(),
                ..Default::default()
            },
        ))
        .id();

    gate_id_map.insert(id, GateEntity::from(entity));
    GateEntity::from(entity)
}
