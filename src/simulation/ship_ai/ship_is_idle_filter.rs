use crate::components::Ship;
use crate::simulation::ship_ai::tasks::{
    AwaitingSignal, ExchangeWares, HarvestGas, MineAsteroid, RequestAccess, UseGate,
};
use crate::simulation::ship_ai::MoveToEntity;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};

#[derive(QueryFilter)]
#[allow(clippy::type_complexity)]
pub struct ShipIsIdleFilter {
    tuple: (
        With<Ship>,
        Without<ExchangeWares>,
        Without<MoveToEntity>,
        Without<UseGate>,
        Without<MineAsteroid>,
        Without<HarvestGas>,
        Without<AwaitingSignal>,
        Without<RequestAccess>,
    ),
}
