use crate::utility::ship_task::ShipTask;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};
use common::types::ship_tasks::*;
use common::{components, impl_all_task_kinds};

macro_rules! impl_task_events {
    ($(($variant:ident, $snake_case_variant:ident)),*) => {
        /// Filters out all ships which have a [ShipTask]<TaskData> component.
        #[derive(QueryFilter)]
        pub struct ShipIsIdleFilter {
            tuple: (
                With<components::Ship>,
                $(
                    Without<ShipTask<$variant>>,
                )*
            ),
        }
    }
}

impl_all_task_kinds!(impl_task_events);
