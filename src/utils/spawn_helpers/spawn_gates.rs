use bevy::core::Name;
use bevy::prelude::{Commands, CubicCurve, Query, SpriteBundle, Vec2};

use crate::components::{
    ConstantOrbit, Gate, GateConnectionComponent, MovingGateConnection, Sector, SectorFeature,
    SelectableEntity,
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

    spawn_gate_connection(
        commands,
        &from_curve,
        &from_sector,
        &to_sector,
        from_gate,
        to_gate,
    );

    from_sector.add_gate(commands, from_pos.sector, from_gate, to_pos.sector, to_gate);
    to_sector.add_gate(commands, to_pos.sector, to_gate, from_pos.sector, from_gate);
}

fn spawn_gate_connection(
    commands: &mut Commands,
    from_to_curve: &CubicCurve<Vec2>,
    from_sector: &Sector,
    to_sector: &Sector,
    from: GateEntity,
    to: GateEntity,
) {
    let mut entity_commands = commands.spawn(GateConnectionComponent::new(from, to, from_to_curve));

    match (&from_sector.feature, &to_sector.feature) {
        (SectorFeature::Star, _) | (_, SectorFeature::Star) => {
            entity_commands.insert(MovingGateConnection);
        }
        (_, _) => {}
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

    match from.feature {
        SectorFeature::Void => {}
        SectorFeature::Star => {
            // TODO!
            let radius = pos.local_position.length();
            entity_commands.insert(ConstantOrbit::new(
                pos.local_position.to_angle(),
                radius,
                helpers::calculate_orbit_velocity(radius, 100.0),
            ));
        }
        SectorFeature::Asteroids(_) => {}
    }

    let entity = entity_commands.id();

    gate_id_map.insert(id, GateEntity::from(entity));
    GateEntity::from(entity)
}
