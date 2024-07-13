use crate::gizmos::gate_connection::{
    draw_gate_connections, on_setup_gate_connection, GateConnectionGizmos, SetupGateConnectionEvent,
};
use crate::gizmos::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use crate::gizmos::selected_ship_tasks;
use crate::map_layout::MapLayout;
use bevy::app::Update;
use bevy::prelude::{on_event, App, AppGizmoBuilder, IntoSystemConfigs, Plugin, Startup};

pub struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<SectorOutlineGizmos>()
            .init_gizmo_group::<GateConnectionGizmos>()
            .init_gizmo_group::<selected_ship_tasks::SelectedShipTaskGizmos>()
            .add_event::<SetupGateConnectionEvent>()
            .add_systems(Startup, selected_ship_tasks::configure)
            .add_systems(
                Update,
                (
                    draw_sector_outlines,
                    draw_gate_connections,
                    selected_ship_tasks::draw_selected_ship_task,
                    on_setup_gate_connection.run_if(on_event::<SetupGateConnectionEvent>()),
                ),
            );
    }
}
