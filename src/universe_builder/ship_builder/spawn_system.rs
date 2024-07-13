use crate::components::Sector;
use crate::persistence::SectorIdMap;
use crate::universe_builder::ship_builder::data_resource::ShipSpawnData;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res};

pub fn spawn_all_ships(
    mut commands: Commands,
    spawn_data: Res<ShipSpawnData>,
    mut sectors: Query<&mut Sector>,
    sprites: Res<SpriteHandles>,
    sector_id_map: Res<SectorIdMap>,
) {
    for builder in &spawn_data.ships {
        builder.build(&mut commands, &mut sectors, &sprites, &sector_id_map);
    }

    commands.remove_resource::<ShipSpawnData>();
}
