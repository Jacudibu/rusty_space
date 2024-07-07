use crate::components::Ship;
use crate::ship_ai::tasks::{ExchangeWares, MineAsteroid, UseGate};
use crate::ship_ai::MoveToEntity;
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
    ),
}
