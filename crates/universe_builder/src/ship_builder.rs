use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Deref, DerefMut, Query, Res};
use common::components::Sector;
use common::components::ship_velocity::ShipVelocity;
use common::session_data::{ShipConfigId, ShipConfigurationManifest};
use common::types::auto_mine_state::AutoMineState;
use common::types::behavior_builder::BehaviorBuilder;
use common::types::entity_id_map::{SectorIdMap, ShipIdMap};
use common::types::local_hex_position::LocalHexPosition;
use common::types::persistent_entity_id::PersistentShipId;
use entity_spawners::spawn_ship::spawn_ship;
use persistence::data::{
    AutoMineStateSaveData, InventorySaveData, SaveDataCollection, ShipBehaviorSaveData,
    ShipSaveData,
};

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,
}

#[derive(Deref, DerefMut, Default)]
pub struct ShipBuilder {
    pub data: Vec<ShipSaveData>,
}

impl ShipBuilder {
    pub fn add(
        &mut self,
        config_id: ShipConfigId,
        position: LocalHexPosition,
        rotation: f32,
        name: String,
        behavior: ShipBehaviorSaveData,
    ) -> &mut ShipSaveData {
        self.push(ShipSaveData {
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

    pub fn build(self) -> SaveDataCollection<ShipSaveData> {
        SaveDataCollection { data: self.data }
    }
}

pub fn spawn_all(data: Res<SaveDataCollection<ShipSaveData>>, mut args: Args) {
    let mut ship_id_map = ShipIdMap::new();
    for data in &data.data {
        spawn_ship(
            &mut args.commands,
            data.id,
            data.name.clone(),
            &mut args.sectors,
            args.sector_id_map.id_to_entity()[&data.position.sector],
            data.position.local_position,
            data.rotation_degrees,
            ShipVelocity {
                forward: data.forward_velocity,
                angular: data.angular_velocity,
            },
            convert_behavior_save_data_to_builder_data(data.behavior),
            &mut ship_id_map,
            args.ship_configurations.get_by_id(&data.config_id).unwrap(),
        );
    }

    args.commands
        .remove_resource::<SaveDataCollection<ShipSaveData>>();
    args.commands.insert_resource(ship_id_map);
}

pub fn convert_behavior_save_data_to_builder_data(value: ShipBehaviorSaveData) -> BehaviorBuilder {
    match value {
        ShipBehaviorSaveData::AutoTrade => BehaviorBuilder::AutoTrade,
        ShipBehaviorSaveData::AutoConstruct => BehaviorBuilder::AutoConstruct,
        ShipBehaviorSaveData::AutoMine { mined_ore, state } => BehaviorBuilder::AutoMine {
            mined_ore,
            state: convert_auto_mine_state(state),
        },
        ShipBehaviorSaveData::AutoHarvest {
            harvested_gas,
            state,
        } => BehaviorBuilder::AutoHarvest {
            harvested_gas,
            state: convert_auto_mine_state(state),
        },
    }
}

fn convert_auto_mine_state(state: AutoMineStateSaveData) -> AutoMineState {
    match state {
        AutoMineStateSaveData::Mining => AutoMineState::Mining,
        AutoMineStateSaveData::Trading => AutoMineState::Trading,
    }
}
