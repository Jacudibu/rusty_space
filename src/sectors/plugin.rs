use crate::sectors::gate::spawn_gates;
use crate::sectors::gate_connection::{
    draw_gate_connections, on_setup_gate_connection, GateConnectionGizmos, SetupGateConnectionEvent,
};
use crate::sectors::map_layout::MapLayout;
use crate::sectors::sector::spawn_sector;
use crate::sectors::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use crate::sectors::{Sector, SectorEntity};
use crate::utils::SectorPosition;
use crate::SpriteHandles;
use bevy::app::Update;
use bevy::prelude::{
    on_event, App, AppGizmoBuilder, Commands, EventWriter, IntoSystemConfigs, Plugin, Query, Res,
    Resource, Vec2,
};
use hexx::Hex;

pub struct SectorPlugin;
impl Plugin for SectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<SectorOutlineGizmos>()
            .init_gizmo_group::<GateConnectionGizmos>()
            .add_event::<SetupGateConnectionEvent>()
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

#[derive(Resource)]
pub struct DebugSectors {
    pub center: SectorEntity,
    pub right: SectorEntity,
    pub top_right: SectorEntity,
    pub bottom_left: SectorEntity,
}

pub fn spawn_test_sectors(mut commands: Commands, map_layout: Res<MapLayout>) {
    let center = Hex::ZERO;
    let right = Hex::new(1, 0);
    let top_right = Hex::new(0, 1);
    let bottom_left = Hex::new(0, -1);

    let center_sector = spawn_sector(&mut commands, &map_layout.hex_layout, center);
    let right_sector = spawn_sector(&mut commands, &map_layout.hex_layout, right);
    let top_right_sector = spawn_sector(&mut commands, &map_layout.hex_layout, top_right);
    let bottom_left_sector = spawn_sector(&mut commands, &map_layout.hex_layout, bottom_left);

    commands.insert_resource(DebugSectors {
        center: center_sector,
        right: right_sector,
        top_right: top_right_sector,
        bottom_left: bottom_left_sector,
    });
}

pub fn spawn_test_gates(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut gate_connection_events: EventWriter<SetupGateConnectionEvent>,
    mut sectors: Query<&mut Sector>,
    debug_sectors: Res<DebugSectors>,
) {
    spawn_gates(
        &mut commands,
        &mut sectors,
        &sprites,
        SectorPosition {
            sector: debug_sectors.center,
            local_position: Vec2::new(250.0, 0.0),
        },
        SectorPosition {
            sector: debug_sectors.right,
            local_position: Vec2::new(-250.0, 0.0),
        },
        &mut gate_connection_events,
    );

    spawn_gates(
        &mut commands,
        &mut sectors,
        &sprites,
        SectorPosition {
            sector: debug_sectors.right,
            local_position: Vec2::new(-200.0, 130.0),
        },
        SectorPosition {
            sector: debug_sectors.top_right,
            local_position: Vec2::new(200.0, -130.0),
        },
        &mut gate_connection_events,
    );

    spawn_gates(
        &mut commands,
        &mut sectors,
        &sprites,
        SectorPosition {
            sector: debug_sectors.center,
            local_position: Vec2::new(-150.0, -150.0),
        },
        SectorPosition {
            sector: debug_sectors.bottom_left,
            local_position: Vec2::new(200.0, 130.0),
        },
        &mut gate_connection_events,
    );
}
