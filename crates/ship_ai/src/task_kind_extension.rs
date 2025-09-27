use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_metadata::TaskMetaData;
use crate::utility::ship_task::ShipTask;
use bevy::prelude::{EntityCommands, Query, Vec2};
use common::components::task_kind::TaskKind;
use common::impl_all_task_kinds;
use common::simulation_transform::SimulationTransform;
use common::types::ship_tasks::*;

/// Useful extension methods to operate non-generically on [TaskKind]
pub trait TaskKindExt {
    fn can_task_be_cancelled_while_active(&self) -> bool;
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2>;
}

/// [TaskKindExt] methods which aren't getting exported
pub(crate) trait TaskKindExtInternal {
    fn add_task_to_entity(&self, entity_commands: &mut EntityCommands);
}

/// Implements [TaskKindExt] and [TaskKindExtInternal] for all possible [TaskKind] values.
macro_rules! impl_traits {
    ($(($variant:ident, $snake_case_variant:ident)), *) => {
        impl TaskKindExt for TaskKind {
            fn can_task_be_cancelled_while_active(&self) -> bool {
                match self {
                    $(TaskKind::$variant { .. } => $variant::can_task_be_cancelled_while_active()),*
                }
            }

            fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
                match self {
                    $(TaskKind::$variant { data } => data.task_target_position(all_transforms)),*
                }
            }
        }

        impl TaskKindExtInternal for TaskKind {
            fn add_task_to_entity(&self, entity_commands: &mut EntityCommands) {
                match self {
                    $(TaskKind::$variant { data } => {
                        entity_commands.insert(ShipTask::<$variant>::new(data.clone()));
                    }),*
                }
            }
        }
    };
}

impl_all_task_kinds!(impl_traits);
