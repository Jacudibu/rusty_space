use crate::components::Ship;
use crate::ship_ai::tasks::{ExchangeWares, UseGate};
use crate::ship_ai::MoveToEntity;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};

#[derive(QueryFilter)]
pub struct ShipIsIdleFilter {
    tuple: (
        With<Ship>,
        Without<ExchangeWares>,
        Without<MoveToEntity>,
        Without<UseGate>,
    ),
}
