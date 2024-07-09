use crate::components::Sector;
use crate::game_data::GameData;
use crate::hex_to_sector_entity_map::HexToSectorEntityMap;
use crate::universe_builder::station_builder::StationSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

pub fn spawn_all_stations(
    mut commands: Commands,
    spawn_data: Res<StationSpawnData>,
    mut sectors: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    hex_to_sector: Res<HexToSectorEntityMap>,
    game_data: Res<GameData>,
) {
    for builder in &spawn_data.stations {
        builder.build(
            &mut commands,
            &mut sectors,
            &sprites,
            &hex_to_sector,
            &game_data,
        );
    }

    commands.remove_resource::<StationSpawnData>();
}
