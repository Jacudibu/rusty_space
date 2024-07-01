use crate::sectors::gate::{spawn_gates, AllGates, SectorPosition};
use crate::sectors::gate_connection::{
    draw_gate_connections, on_setup_gate_connection, GateConnectionGizmos, SetupGateConnectionEvent,
};
use crate::sectors::sector::{spawn_sector, AllSectors};
use crate::sectors::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use crate::SpriteHandles;
use bevy::app::Update;
use bevy::prelude::{
    on_event, App, AppGizmoBuilder, Commands, EventWriter, IntoSystemConfigs, Plugin, Res,
    Resource, Startup, Vec2,
};
use hexx::{Hex, HexLayout, HexOrientation};

pub struct SectorPlugin;
impl Plugin for SectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<SectorOutlineGizmos>()
            .init_gizmo_group::<GateConnectionGizmos>()
            .add_event::<SetupGateConnectionEvent>()
            .add_systems(Startup, spawn_test_stuff.after(crate::initialize_data))
            .add_systems(
                Update,
                (
                    draw_sector_outlines,
                    draw_gate_connections,
                    on_setup_gate_connection.run_if(on_event::<SetupGateConnectionEvent>()),
                ),
            );
    }
}

fn spawn_test_stuff(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    map_layout: Res<MapLayout>,
    mut gate_connection_events: EventWriter<SetupGateConnectionEvent>,
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
        &mut gate_connection_events,
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
