use crate::components::Sector;
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::persistence::{PersistentShipId, SectorIdMap, ShipIdMap};
use crate::ship_ai::BehaviorBuilder;
use crate::utils::spawn_helpers;
use crate::SpriteHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};

type SaveData = SaveDataCollection<ShipSaveData>;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, &'static mut Sector>,
    sector_id_map: Res<'w, SectorIdMap>,
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
        position: LocalHexPosition,
        rotation: f32,
        name: String,
        behavior: ShipBehaviorSaveData,
    ) -> &mut ShipSaveData {
        self.data.push(ShipSaveData {
            id: PersistentShipId::next(),
            name,
            position,
            rotation,
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
        spawn_helpers::spawn_ship(
            &mut args.commands,
            &args.sprites,
            self.name.clone(),
            &mut args.sectors,
            sector_entity,
            self.position.position,
            self.rotation,
            &BehaviorBuilder::from(self.behavior),
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
