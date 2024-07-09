mod a_star;
mod create_tasks_following_path;
mod path_element;
mod search_node;

use crate::components::Sector;
use crate::utils::SectorEntity;
use bevy::math::Vec3;
use bevy::prelude::{Query, Transform};

pub use {create_tasks_following_path::create_tasks_to_follow_path, path_element::PathElement};

/// Returns the fastest gate-path between `from` and `to`.   
pub fn find_path(
    sectors: &Query<&Sector>,
    gate_positions: &Query<&Transform>,
    from: SectorEntity,
    from_position: Vec3,
    to: SectorEntity,
) -> Option<Vec<PathElement>> {
    a_star::a_star(sectors, gate_positions, from, from_position, to)
}