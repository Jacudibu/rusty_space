use crate::components::SectorComponent;
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{PersistentShipId, SectorIdMap, ShipIdMap};
use crate::session_data::{ShipConfigId, ShipConfigurationManifest};
use crate::simulation::prelude::ShipVelocity;
use crate::simulation::ship_ai::BehaviorBuilder;
use crate::utils::entity_spawners;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};

type SaveData = SaveDataCollection<ShipSaveData>;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sectors: Query<'w, 's, &'static mut SectorComponent>,
    sector_id_map: Res<'w, SectorIdMap>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,
}

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut ship_id_map = ShipIdMap::new();
    for builder in &data.data {
        builder.build(&mut args, &mut ship_id_map);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(ship_id_map);
}

impl SaveData {
    pub fn add(
        &mut self,
        config_id: ShipConfigId,
        position: LocalHexPosition,
        rotation: f32,
        name: String,
        behavior: ShipBehaviorSaveData,
    ) -> &mut ShipSaveData {
        self.data.push(ShipSaveData {
            id: PersistentShipId::next(),
            config_id,
            name,
            position,
            rotation_degrees: rotation,
            behavior,
            forward_velocity: 0.0,
            angular_velocity: 0.0,
            task_queue: Vec::new(), // TODO
            inventory: InventorySaveData { items: Vec::new() },
        });
        self.data.last_mut().unwrap()
    }
}

impl ShipSaveData {
    pub fn build(&self, args: &mut Args, ship_id_map: &mut ShipIdMap) {
        let sector_entity = args.sector_id_map.id_to_entity()[&self.position.sector];
        entity_spawners::spawn_ship(
            &mut args.commands,
            self.id,
            self.name.clone(),
            &mut args.sectors,
            sector_entity,
            self.position.position,
            self.rotation_degrees,
            ShipVelocity {
                forward: self.forward_velocity,
                angular: self.angular_velocity,
            },
            BehaviorBuilder::from(self.behavior),
            ship_id_map,
            args.ship_configurations.get_by_id(&self.config_id).unwrap(),
        );
    }
}

impl From<ShipBehaviorSaveData> for BehaviorBuilder {
    fn from(value: ShipBehaviorSaveData) -> Self {
        match value {
            ShipBehaviorSaveData::AutoTrade { next_idle_update } => {
                BehaviorBuilder::AutoTrade { next_idle_update }
            }
            ShipBehaviorSaveData::AutoConstruct { next_idle_update } => {
                BehaviorBuilder::AutoConstruct { next_idle_update }
            }
            ShipBehaviorSaveData::AutoMine {
                next_idle_update,
                mined_ore,
                state,
            } => BehaviorBuilder::AutoMine {
                next_idle_update,
                mined_ore,
                state,
            },
            ShipBehaviorSaveData::AutoHarvest {
                next_idle_update,
                harvested_gas,
                state,
            } => BehaviorBuilder::AutoHarvest {
                next_idle_update,
                harvested_gas,
                state,
            },
        }
    }
}
