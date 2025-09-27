use bevy::prelude::{Entity, Query, Vec2};
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::ShipTaskData;

pub trait TaskMetaData<'w, 's, TaskData: ShipTaskData> {
    /// Determines the target position of the provided [TaskData].
    /// # Returns
    /// - [None] if the task doesn't specify (or care) about a target position
    /// - [Some]<[Vec2]> in case the task specifies a target position
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2>;
}

/// Returns the Entity's [SimulationTransform] in case it can be found, [None] otherwise.
pub(crate) fn get_entity_global_position(
    all_transforms: &Query<&SimulationTransform>,
    entity: Entity,
) -> Option<Vec2> {
    if let Ok(target) = all_transforms.get(entity) {
        Some(target.translation)
    } else {
        None
    }
}
