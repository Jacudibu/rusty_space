use crate::components::Sector;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::universe_builder::ship_builder::data_resource::ShipSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

pub fn spawn_all_ships(
    mut commands: Commands,
    spawn_data: Res<ShipSpawnData>,
    mut sectors: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    hex_to_sector: Res<HexToSectorEntityMap>,
) {
    for builder in &spawn_data.ships {
        builder.build(&mut commands, &mut sectors, &sprites, &hex_to_sector);
    }

    commands.remove_resource::<ShipSpawnData>();
}
