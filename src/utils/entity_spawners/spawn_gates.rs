use crate::SpriteHandles;
use crate::persistence::{GateIdMap, PersistentGateId};
use crate::simulation::prelude::SimulationScale;
use crate::utils::SectorPosition;
use crate::utils::{CelestialMass, GateEntity};
use bevy::prelude::{Commands, CubicCurve, Name, Query, Sprite, Vec2};
use common::components::{
    ConstantOrbit, Gate, GateConnection, MovingGateConnection, Sector, SectorWithCelestials,
    SelectableEntity,
};
use common::constants;
use common::simulation_transform::SimulationTransform;
use common::types::polar_coordinates::PolarCoordinates;

#[allow(clippy::too_many_arguments)]
pub fn spawn_gate_pair(
    commands: &mut Commands,
    gate_id_map: &mut GateIdMap,
    sectors: &mut Query<(&mut Sector, Option<&SectorWithCelestials>)>,
    sprites: &SpriteHandles,
    from_id: PersistentGateId,
    from_pos: SectorPosition,
    to_id: PersistentGateId,
    to_pos: SectorPosition,
) {
    let [
        (mut from_sector, from_celestial),
        (mut to_sector, to_celestial),
    ] = sectors
        .get_many_mut([from_pos.sector.into(), to_pos.sector.into()])
        .unwrap();

    let (from_curve, to_curve) = GateConnection::calculate_curves_from_local_positions(
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
        from_celestial.map(|x| &x.center_mass),
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
        to_celestial.map(|x| &x.center_mass),
    );

    spawn_gate_connection(
        commands,
        &from_curve,
        from_gate,
        to_gate,
        from_celestial.is_some() || to_celestial.is_some(),
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
    let mut entity_commands = commands.spawn(GateConnection::new(from, to, from_to_curve));

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
    center_mass: Option<&CelestialMass>,
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
        Sprite::from_image(sprites.gate.clone()),
        simulation_transform.as_bevy_transform(constants::z_layers::GATE),
        simulation_transform,
        SimulationScale::default(),
    ));

    if let Some(center_mass) = center_mass {
        let polar_coordinates = PolarCoordinates::from_cartesian(&pos.local_position);
        entity_commands.insert(ConstantOrbit::new(polar_coordinates, center_mass));
    }

    let entity = entity_commands.id();

    gate_id_map.insert(id, GateEntity::from(entity));
    GateEntity::from(entity)
}
