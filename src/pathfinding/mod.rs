mod a_star;
mod path_element;
mod search_node;
pub mod surrounding_sector_search;

use crate::utils::SectorEntity;
use bevy::prelude::{Query, Vec2};
use common::components::Sector;

use common::simulation_transform::SimulationTransform;
pub use path_element::PathElement;

/// Returns the fastest gate-path between `from` and `to`.
#[must_use]
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
