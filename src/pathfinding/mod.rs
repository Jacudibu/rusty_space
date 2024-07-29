mod a_star;
mod create_tasks_following_path;
mod path_element;
mod search_node;
pub mod surrounding_sector_search;

use crate::components::Sector;
use crate::utils::SectorEntity;
use bevy::prelude::{Query, Vec2};

use crate::simulation::transform::simulation_transform::SimulationTransform;
pub use {create_tasks_following_path::create_tasks_to_follow_path, path_element::PathElement};

/// Returns the fastest gate-path between `from` and `to`.   
pub fn find_path(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&SimulationTransform>,
    from: SectorEntity,
    from_position: Vec2,
    to: SectorEntity,
    to_position: Option<Vec2>,
) -> Option<Vec<PathElement>> {
    a_star::a_star(
        sectors,
        gate_positions,
        from,
        from_position,
        to,
        to_position,
    )
}
