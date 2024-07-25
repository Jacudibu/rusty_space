use crate::gizmos::{gate_connection, orbits, sector_outlines, selected_ship_tasks};
use crate::map_layout::MapLayout;
use bevy::app::Update;
use bevy::prelude::{App, AppGizmoBuilder, Plugin, Startup};

pub struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<sector_outlines::SectorOutlineGizmos>()
            .init_gizmo_group::<gate_connection::GateConnectionGizmos>()
            .init_gizmo_group::<selected_ship_tasks::SelectedShipTaskGizmos>()
            .init_gizmo_group::<orbits::OrbitLineGizmos>()
            .add_systems(Startup, (selected_ship_tasks::configure, orbits::configure))
            .add_systems(
                Update,
                (
                    sector_outlines::draw_sector_outlines,
                    gate_connection::draw_gate_connections,
                    orbits::draw_orbit_circles,
                    selected_ship_tasks::draw_selected_ship_task,
                ),
            );
    }
}
