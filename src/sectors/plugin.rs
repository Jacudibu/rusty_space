use crate::sectors::gate::{spawn_gates, AllGates, SectorPosition};
use crate::sectors::gate_lines::{draw_gate_lines, GateLineGizmos};
use crate::sectors::sector::{spawn_sector, AllSectors};
use crate::sectors::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use crate::SpriteHandles;
use bevy::app::Update;
use bevy::prelude::{
    App, AppGizmoBuilder, Commands, IntoSystemConfigs, Plugin, Res, Resource, Startup, Vec2,
};
use hexx::{Hex, HexLayout, HexOrientation};

pub struct SectorPlugin;
impl Plugin for SectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<SectorOutlineGizmos>()
            .init_gizmo_group::<GateLineGizmos>()
            .add_systems(Startup, spawn_test_stuff.after(crate::initialize_data))
            .add_systems(Update, (draw_sector_outlines, draw_gate_lines));
    }
}

fn spawn_test_stuff(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    map_layout: Res<MapLayout>,
) {
    let mut all_sectors = AllSectors::default();
    let mut all_gates = AllGates::default();

    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let bottom_right = Hex::new(0, 1);
    let top_left = Hex::new(0, -1);

    spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        center,
        &mut all_sectors,
    );
    spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        right,
        &mut all_sectors,
    );
    spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        bottom_right,
        &mut all_sectors,
    );
    spawn_sector(
        &mut commands,
        &map_layout.hex_layout,
        top_left,
        &mut all_sectors,
    );

    spawn_gates(
        &mut commands,
        &sprites,
        SectorPosition {
            sector: center,
            position: Vec2::new(250.0, 0.0),
        },
        SectorPosition {
            sector: right,
            position: Vec2::new(-250.0, 0.0),
        },
        &mut all_sectors,
        &mut all_gates,
    );

    commands.insert_resource(all_sectors);
    commands.insert_resource(all_gates);
}

#[derive(Resource)]
pub struct MapLayout {
    pub hex_layout: HexLayout,
}

impl Default for MapLayout {
    fn default() -> Self {
        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: hexx::Vec2::splat(500.0),
                ..Default::default()
            },
        }
    }
}
