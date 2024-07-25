use bevy::core::Name;
use bevy::prelude::{Commands, CubicCurve, Has, Query, SpriteBundle, Vec2};

use crate::components::{
    ConstantOrbit, Gate, GateConnectionComponent, MovingGateConnection, Sector,
    SectorStarComponent, SelectableEntity,
};
use crate::persistence::{GateIdMap, PersistentGateId};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::spawn_helpers::helpers;
use crate::utils::GateEntity;
use crate::utils::SectorPosition;
use crate::{constants, SpriteHandles};

#[allow(clippy::too_many_arguments)]
pub fn spawn_gate_pair(
    commands: &mut Commands,
    gate_id_map: &mut GateIdMap,
    sector_query: &mut Query<(&mut Sector, Has<SectorStarComponent>)>,
    sprites: &SpriteHandles,
    from_id: PersistentGateId,
    from_pos: SectorPosition,
    to_id: PersistentGateId,
    to_pos: SectorPosition,
) {
    let [(mut from_sector, from_has_star), (mut to_sector, to_has_star)] = sector_query
        .get_many_mut([from_pos.sector.into(), to_pos.sector.into()])
        .unwrap();

    let (from_curve, to_curve) = GateConnectionComponent::calculate_curves_from_local_positions(
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
        from_curve.clone(),
        from_has_star,
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
        to_has_star,
    );

    spawn_gate_connection(
        commands,
        &from_curve,
        from_gate,
        to_gate,
        from_has_star || to_has_star,
    );

    from_sector.add_gate(commands, from_pos.sector, from_gate, to_pos.sector, to_gate);
    to_sector.add_gate(commands, to_pos.sector, to_gate, from_pos.sector, from_gate);
}

fn spawn_gate_connection(
    commands: &mut Commands,
    from_to_curve: &CubicCurve<Vec2>,
    from: GateEntity,
    to: GateEntity,
    has_orbital_mechanics: bool,
) {
    let mut entity_commands = commands.spawn(GateConnectionComponent::new(from, to, from_to_curve));

    if has_orbital_mechanics {
        entity_commands.insert(MovingGateConnection);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_gate(
    commands: &mut Commands,
    id: PersistentGateId,
    gate_id_map: &mut GateIdMap,
    sprites: &SpriteHandles,
    pos: &SectorPosition,
    from: &mut Sector,
    to: &Sector,
    ship_curve: CubicCurve<Vec2>,
    has_orbital_mechanics: bool,
) -> GateEntity {
    let simulation_transform =
        SimulationTransform::from_translation(from.world_pos + pos.local_position);
    let mut entity_commands = commands.spawn((
        Name::new(format!(
            "Gate [{},{}] -> [{},{}]",
            from.coordinate.x, from.coordinate.y, to.coordinate.x, to.coordinate.y
        )),
        Gate::new(id, ship_curve),
        SelectableEntity::Gate,
        SpriteBundle {
            transform: simulation_transform.as_transform(constants::GATE_LAYER),
            texture: sprites.gate.clone(),
            ..Default::default()
        },
        simulation_transform,
    ));

    if has_orbital_mechanics {
        // TODO!
        let radius = pos.local_position.length();
        entity_commands.insert(ConstantOrbit::new(
            pos.local_position.to_angle(),
            radius,
            helpers::calculate_orbit_velocity(radius, 100.0),
        ));
    }

    let entity = entity_commands.id();

    gate_id_map.insert(id, GateEntity::from(entity));
    GateEntity::from(entity)
}
