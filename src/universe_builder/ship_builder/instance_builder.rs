use crate::components::Sector;
use crate::persistence::SectorIdMap;
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
        sector_id_map: &SectorIdMap,
    ) {
        let sector_entity = sector_id_map.id_to_entity()[&self.position.sector];
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
