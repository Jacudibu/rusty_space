use crate::components;
use crate::simulation::ship_ai::tasks;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};

#[derive(QueryFilter)]
#[allow(clippy::type_complexity)]
pub struct ShipIsIdleFilter {
    tuple: (
        With<components::Ship>,
        Without<tasks::AwaitingSignal>,
        Without<tasks::Build>,
        Without<tasks::DockAtEntity>,
        Without<tasks::ExchangeWares>,
        Without<tasks::HarvestGas>,
        Without<tasks::MineAsteroid>,
        Without<tasks::MoveToEntity>,
        Without<tasks::RequestAccess>,
        Without<tasks::Undock>,
        Without<tasks::UseGate>,
    ),
}
