use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::universe_builder::station_builder::instance_builder::StationSpawnDataInstanceBuilder;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct StationSpawnData {
    pub stations: Vec<StationSpawnDataInstanceBuilder>,
}

impl StationSpawnData {
    pub fn add(
        &mut self,
        position: LocalHexPosition,
        name: String,
    ) -> &mut StationSpawnDataInstanceBuilder {
        self.stations
            .push(StationSpawnDataInstanceBuilder::new(position, name));
        self.stations.last_mut().unwrap()
    }
}
