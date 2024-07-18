use crate::components::Sector;
use crate::persistence::data::v1::*;
use crate::persistence::{PersistentShipId, SectorIdMap, ShipIdMap};
use crate::ship_ai::BehaviorBuilder;
use crate::universe_builder::LocalHexPosition;
use crate::utils::{spawn_helpers, SimulationTimestamp};
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

impl SaveDataCollection<ShipSaveData> {
    pub fn add(
        &mut self,
        position: LocalHexPosition,
        rotation: f32,
        name: String,
    ) -> &mut ShipSaveData {
        self.data.push(ShipSaveData {
            id: PersistentShipId::next(),
            name,
            position,
            rotation,
            forward_velocity: 0.0,
            angular_velocity: 0.0,
            behavior: ShipBehaviorSaveData::AutoTrade {
                next_idle_update: SimulationTimestamp::MIN,
            },
            task_queue: Vec::new(),
            inventory: InventorySaveData { items: Vec::new() },
        });
        self.data.last_mut().unwrap()
    }

    pub fn spawn_all(
        &self,
        mut commands: Commands,
        mut sectors: Query<&mut Sector>,
        sprites: Res<SpriteHandles>,
        sector_id_map: Res<SectorIdMap>,
    ) {
        let mut ship_id_map = ShipIdMap::new();
        for builder in &self.data {
            builder.build(
                &mut commands,
                &mut sectors,
                &sprites,
                &sector_id_map,
                &mut ship_id_map,
            );
        }

        commands.insert_resource(ship_id_map);
    }
}

impl ShipSaveData {
    pub fn build(
        &self,
        commands: &mut Commands,
        sectors: &mut Query<&mut Sector>,
        sprites: &SpriteHandles,
        sector_id_map: &SectorIdMap,
        ship_id_map: &mut ShipIdMap,
    ) {
        let sector_entity = sector_id_map.id_to_entity()[&self.position.sector];
        spawn_helpers::spawn_ship(
            commands,
            sprites,
            self.name.clone(),
            sectors,
            sector_entity,
            self.position.position,
            self.rotation,
            BehaviorBuilder::from(self.behavior.clone()),
            ship_id_map,
        );
    }
}

impl From<ShipBehaviorSaveData> for BehaviorBuilder {
    fn from(value: ShipBehaviorSaveData) -> Self {
        match value {
            ShipBehaviorSaveData::AutoTrade { next_idle_update } => {
                BehaviorBuilder::AutoTrade { next_idle_update }
            }
            ShipBehaviorSaveData::AutoMine {
                next_idle_update,
                state,
            } => BehaviorBuilder::AutoMine {
                next_idle_update,
                state,
            },
        }
    }
}
