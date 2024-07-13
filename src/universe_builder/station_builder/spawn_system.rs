use crate::components::Sector;
use crate::game_data::GameData;
use crate::persistence::SectorIdMap;
use crate::universe_builder::station_builder::StationSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

pub fn spawn_all_stations(
    mut commands: Commands,
    spawn_data: Res<StationSpawnData>,
    mut sectors: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    sector_id_map: Res<SectorIdMap>,
    game_data: Res<GameData>,
) {
    for builder in &spawn_data.stations {
        builder.build(
            &mut commands,
            &mut sectors,
            &sprites,
            &sector_id_map,
            &game_data,
        );
    }

    commands.remove_resource::<StationSpawnData>();
}
