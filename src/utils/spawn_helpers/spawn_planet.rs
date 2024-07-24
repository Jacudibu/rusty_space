use crate::components::{ConstantOrbit, Planet, Sector};
use crate::persistence::{PersistentPlanetId, PlanetIdMap};
use crate::simulation::ship_ai::AutoTradeBehavior;
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{PlanetEntity, SectorEntity};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Rot2, SpriteBundle};

#[allow(clippy::too_many_arguments)]
pub fn spawn_planet(
    commands: &mut Commands,
    planet_id_map: &mut PlanetIdMap,
    sprites: &SpriteHandles,
    name: String,
    sectors: &mut Query<&mut Sector>,
    sector_entity: SectorEntity,
    orbit_distance: f32,
    current_orbit_angle: f32,
    velocity: f32,
    mass: u32,
) {
    let mut sector_data = sectors.get_mut(sector_entity.into()).unwrap();

    let planet_id = PersistentPlanetId::next();
    let planet = Planet::new(planet_id, mass);

    // TODO: Figure out whether orbit should always be 0.0,
    //      if yes, this should be deleted, if not, this should be persisted in sector
    let orbit_around = Vec2::ZERO;

    // TODO: calculate that from distance and angle
    let local_position = Vec2::ZERO;

    let simulation_transform =
        SimulationTransform::new(sector_data.world_pos + local_position, Rot2::IDENTITY, 1.0);

    let entity = commands
        .spawn((
            Name::new(name),
            // SelectableEntity::Planet,
            AutoTradeBehavior::default(),
            ConstantOrbit::new(orbit_around, current_orbit_angle, orbit_distance, velocity),
            SpriteBundle {
                texture: sprites.planet.clone(),
                transform: simulation_transform.as_transform(constants::PLANET_AND_STARS_LAYER),
                ..default()
            },
            simulation_transform,
            planet,
        ))
        .id();

    let planet_entity = PlanetEntity::from(entity);
    planet_id_map.insert(planet_id, planet_entity);
    sector_data.add_planet(commands, sector_entity, planet_entity);
}
