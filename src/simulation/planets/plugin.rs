use crate::simulation::planets::orbit_directions::OrbitDirections;
use crate::simulation::planets::orbit_system::orbit_system;
use bevy::app::{App, FixedPostUpdate, Plugin};

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OrbitDirections>()
            .add_systems(FixedPostUpdate, orbit_system);
    }
}
