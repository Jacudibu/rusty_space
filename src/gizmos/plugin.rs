use crate::gizmos::gate_connection::{
    draw_gate_connections, on_setup_gate_connection, GateConnectionGizmos, SetupGateConnectionEvent,
};
use crate::gizmos::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use crate::map_layout::MapLayout;
use bevy::app::Update;
use bevy::prelude::{on_event, App, AppGizmoBuilder, IntoSystemConfigs, Plugin};
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
