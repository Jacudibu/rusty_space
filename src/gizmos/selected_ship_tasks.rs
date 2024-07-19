use crate::components::Gate;
use crate::entity_selection::Selected;
use crate::simulation::ship_ai::{TaskInsideQueue, TaskQueue};
use bevy::prelude::{
    GizmoConfigGroup, GizmoConfigStore, Gizmos, Query, Reflect, ResMut, Srgba, Transform, With,
};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SelectedShipTaskGizmos;

const GIZMO_COLOR: Srgba = bevy::color::palettes::css::CORNFLOWER_BLUE;

pub fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<SelectedShipTaskGizmos>();
    config.line_width = 4.0;
}

pub fn draw_selected_ship_task(
    mut gizmos: Gizmos<SelectedShipTaskGizmos>,
    selected_ships: Query<(&TaskQueue, &Transform), With<Selected>>,
    all_transforms: Query<&Transform>,
    all_gates: Query<&Gate>,
) {
    for (tasks, transform) in selected_ships.iter() {
        let mut current_position = transform.translation;
        for x in &tasks.queue {
            match x {
                TaskInsideQueue::ExchangeWares { .. } => {}
                TaskInsideQueue::MoveToEntity { target, .. } => {
                    let target_position = all_transforms.get(target.into()).unwrap().translation;
                    gizmos.line(current_position, target_position, GIZMO_COLOR);
                    current_position = target_position;
                }
                TaskInsideQueue::UseGate { enter_gate, .. } => {
                    let gate = all_gates.get(enter_gate.into()).unwrap();
                    gizmos.linestrip_2d(gate.transit_curve.iter_positions(10), GIZMO_COLOR);
                    current_position = gate.transit_curve.position(1.0).extend(0.0);
                }
                TaskInsideQueue::MineAsteroid { target, .. } => {
                    let asteroid_pos = all_transforms.get(target.into()).unwrap().translation;
                    gizmos.line(current_position, asteroid_pos, GIZMO_COLOR);
                }
            }
        }
    }
}
