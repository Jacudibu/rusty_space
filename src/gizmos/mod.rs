use crate::gizmos::gate_gizmos::GateGizmoPlugin;
use crate::gizmos::orbit_gizmos::OrbitGizmoPlugin;
use crate::gizmos::sector_gizmos::SectorGizmoPlugin;
use crate::gizmos::ship_task_gizmos::ShipTaskGizmoPlugin;
use bevy::prelude::{App, Plugin};

mod gate_gizmos;
mod orbit_gizmos;
mod sector_gizmos;
mod ship_task_gizmos;

pub struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GateGizmoPlugin,
            OrbitGizmoPlugin,
            SectorGizmoPlugin,
            ShipTaskGizmoPlugin,
        ));
    }
}
