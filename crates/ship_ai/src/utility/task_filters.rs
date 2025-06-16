use crate::utility::ship_task::ShipTask;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::{With, Without};
use common::components;
use common::types::ship_tasks::{
    AwaitingSignal, Construct, DockAtEntity, ExchangeWares, HarvestGas, MineAsteroid, MoveToEntity,
    MoveToPosition, RequestAccess, Undock, UseGate,
};

#[derive(QueryFilter)]
#[allow(clippy::type_complexity)]
pub struct ShipIsIdleFilter {
    tuple: (
        With<components::Ship>,
        Without<ShipTask<AwaitingSignal>>,
        Without<ShipTask<Construct>>,
        Without<ShipTask<DockAtEntity>>,
        Without<ShipTask<ExchangeWares>>,
        Without<ShipTask<HarvestGas>>,
        Without<ShipTask<MineAsteroid>>,
        Without<ShipTask<MoveToEntity>>,
        Without<ShipTask<MoveToPosition>>,
        Without<ShipTask<RequestAccess>>,
        Without<ShipTask<Undock>>,
        Without<ShipTask<UseGate>>,
    ),
}
