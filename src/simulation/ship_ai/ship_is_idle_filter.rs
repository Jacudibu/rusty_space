use crate::components;
use crate::simulation::ship_ai::tasks;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};

#[derive(QueryFilter)]
#[allow(clippy::type_complexity)]
pub struct ShipIsIdleFilter {
    tuple: (
        With<components::Ship>,
        Without<tasks::ExchangeWares>,
        Without<tasks::MoveToEntity>,
        Without<tasks::UseGate>,
        Without<tasks::MineAsteroid>,
        Without<tasks::HarvestGas>,
        Without<tasks::AwaitingSignal>,
        Without<tasks::RequestAccess>,
        Without<tasks::DockAtEntity>,
        Without<tasks::Undock>,
    ),
}
