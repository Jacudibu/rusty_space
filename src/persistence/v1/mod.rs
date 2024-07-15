use crate::components::{BuyOrders, InSector, Inventory, Sector, SellOrders, Ship, Station};
use crate::persistence::AllEntityIdMaps;
use crate::physics::ShipVelocity;
use crate::production::{ProductionComponent, ShipyardComponent};
use crate::ship_ai::TaskQueue;
use crate::utils::AsteroidEntityWithTimestamp;
use bevy::math::Vec2;
use bevy::prelude::{Name, Query, Transform};
use hexx::Hex;
use serde::{Deserialize, Serialize};
use ship_save_data::ShipSaveData;
use station_save_data::StationSaveData;

mod inventory_save_data;
mod ship_save_data;
mod station_save_data;
mod task_save_data;

pub struct AsteroidSaveData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
}

impl AsteroidSaveData {
    pub fn from(asteroid: AsteroidEntityWithTimestamp) {}
}

#[derive(Default)]
pub struct SectorSaveData {
    pub coordinate: Hex,
    pub live_asteroids: Vec<AsteroidSaveData>,
    pub respawning_asteroids: Vec<AsteroidSaveData>,
}

impl SectorSaveData {
    pub fn from(sector: &Sector) -> Self {
        todo!();
        let live_asteroids = sector.asteroids.iter().map(|x| {});

        Self {
            coordinate: sector.coordinate,
            live_asteroids: Vec::new(),
            respawning_asteroids: Vec::new(),
        }
    }
}

pub struct GateSaveData {
    pub from_sector: Hex,
    pub from_position: Vec2,
    pub to_sector: Hex,
    pub to_position: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct UniverseSaveData {
    //sectors: Vec<SectorSaveData>,
    ships: Vec<ShipSaveData>,
    stations: Vec<StationSaveData>,
}

/// For as long as there is no "public test build", save data is not guaranteed to be compatible with older/newer versions of the game.
#[allow(clippy::type_complexity)] // Haha, like, uh, yeah. No.
pub fn save(
    ships: Query<(
        &Ship,
        &Name,
        &Transform,
        &TaskQueue,
        &ShipVelocity,
        &Inventory,
    )>,
    stations: Query<(
        &Station,
        &Name,
        &InSector,
        &Transform,
        &Inventory,
        Option<&ProductionComponent>,
        Option<&ShipyardComponent>,
        Option<&BuyOrders>,
        Option<&SellOrders>,
    )>,
    all_entity_id_maps: AllEntityIdMaps,
) {
    let ship_save_data = ships
        .iter()
        .map(|query_content| ShipSaveData::from(query_content, &all_entity_id_maps))
        .collect();

    let station_save_data = stations
        .iter()
        .map(|query_content| StationSaveData::from(query_content, &all_entity_id_maps))
        .collect();

    let save_data = UniverseSaveData {
        ships: ship_save_data,
        stations: station_save_data,
    };
}
