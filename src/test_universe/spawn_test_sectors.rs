use crate::map_layout::MapLayout;
use crate::spawn_helpers::spawn_sector;
use crate::test_universe::plugin::TestSectors;
use bevy::prelude::{Commands, Res};
use hexx::Hex;

pub fn spawn_test_sectors(mut commands: Commands, map_layout: Res<MapLayout>) {
    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let top_right = Hex::new(0, 1);
    let bottom_left = Hex::new(0, -1);

    let center_sector = spawn_sector(&mut commands, &map_layout.hex_layout, center);
    let right_sector = spawn_sector(&mut commands, &map_layout.hex_layout, right);
    let top_right_sector = spawn_sector(&mut commands, &map_layout.hex_layout, top_right);
    let bottom_left_sector = spawn_sector(&mut commands, &map_layout.hex_layout, bottom_left);

    commands.insert_resource(TestSectors {
        center: center_sector,
        right: right_sector,
        top_right: top_right_sector,
        bottom_left: bottom_left_sector,
    });
}
