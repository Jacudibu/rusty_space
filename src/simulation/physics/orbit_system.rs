use crate::components::{ConstantOrbit, InSector, Sector};
use crate::simulation::precomputed_orbit_directions::PrecomputedOrbitDirections;
use crate::simulation::prelude::SimulationTransform;
use bevy::prelude::{Fixed, Query, Res, Time};

pub fn orbit_system(
    time: Res<Time<Fixed>>,
    orbit_directions: Res<PrecomputedOrbitDirections>,
    mut orbits: Query<(&mut ConstantOrbit, &mut SimulationTransform, &InSector)>,
    sectors: Query<&Sector>,
) {
    let delta = time.delta_secs();

    orbits
        .par_iter_mut()
        .for_each(|(mut orbit, mut transform, in_sector)| {
            orbit.advance(delta);
            let sector_pos = sectors.get(in_sector.sector.into()).unwrap().world_pos;

            transform.translation = sector_pos
                + orbit_directions.orbit_position_at(orbit.radius, orbit.rotational_fraction);
        })
}
