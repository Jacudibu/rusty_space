use bevy::math::Vec2;
use bevy::prelude::{Commands, EventWriter, Query, Res};

use crate::components::Sector;
use crate::gizmos::SetupGateConnectionEvent;
use crate::spawn_helpers::spawn_gates;
use crate::test_universe::plugin::TestSectors;
use crate::utils::SectorPosition;
use crate::SpriteHandles;

pub fn spawn_test_gates(
    mut commands: Commands,
    sprites: Res<SpriteHandles>,
    mut gate_connection_events: EventWriter<SetupGateConnectionEvent>,
    mut sectors: Query<&mut Sector>,
    debug_sectors: Res<TestSectors>,
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
