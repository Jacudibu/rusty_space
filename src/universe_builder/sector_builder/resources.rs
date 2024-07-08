use crate::universe_builder::sector_builder::instance_builder::SectorSpawnDataInstanceBuilder;
use crate::utils::SectorEntity;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use hexx::Hex;

#[derive(Resource, Default)]
pub struct SectorSpawnData {
    pub sectors: Vec<SectorSpawnDataInstanceBuilder>,
}

impl SectorSpawnData {
    pub fn add(&mut self, hex: Hex) -> &mut SectorSpawnDataInstanceBuilder {
        self.sectors.push(SectorSpawnDataInstanceBuilder::new(hex));
        self.sectors.last_mut().unwrap()
    }
}

#[derive(Resource)]
pub struct HexToSectorEntityMap {
    pub(crate) map: HashMap<Hex, SectorEntity>,
}
