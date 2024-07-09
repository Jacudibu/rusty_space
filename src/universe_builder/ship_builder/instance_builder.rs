use crate::components::Sector;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::ship_ai::BehaviorBuilder;
use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::utils::spawn_helpers::spawn_ship;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query};

pub struct ShipSpawnDataInstanceBuilder {
    pub name: String,
    pub position: LocalHexPosition,
    pub rotation: f32,
    pub behavior: BehaviorBuilder,
}

impl ShipSpawnDataInstanceBuilder {
    pub fn new(
        position: LocalHexPosition,
        rotation: f32,
        name: String,
        behavior: BehaviorBuilder,
    ) -> Self {
        Self {
            name,
            position,
            rotation,
            behavior,
        }
    }

    pub fn build(
        &self,
        commands: &mut Commands,
        sectors: &mut Query<&mut Sector>,
        sprites: &SpriteHandles,
        hex_to_sector: &HexToSectorEntityMap,
    ) {
        let sector_entity = hex_to_sector.map[&self.position.sector];
        spawn_ship(
            commands,
            sprites,
            self.name.clone(),
            sectors,
            sector_entity,
            self.position.position,
            self.rotation,
            &self.behavior,
        )
    }
}
