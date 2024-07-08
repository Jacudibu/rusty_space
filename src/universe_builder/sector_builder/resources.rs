use crate::universe_builder::sector_builder::instance_builder::SectorSpawnDataInstanceBuilder;
use bevy::prelude::Resource;
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
