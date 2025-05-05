use crate::entity_selection::IsEntitySelected;
use bevy::prelude::{
    GizmoConfigGroup, GizmoConfigStore, Gizmos, Query, Reflect, ResMut, Srgba, Transform, With,
};
use common::components::Gate;
use common::components::task_queue::{TaskInsideQueue, TaskQueue};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SelectedShipTaskGizmos;

const GIZMO_COLOR: Srgba = bevy::color::palettes::css::CORNFLOWER_BLUE;

pub fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<SelectedShipTaskGizmos>();
    config.line.width = 4.0;
}

pub fn draw_selected_ship_task(
    mut gizmos: Gizmos<SelectedShipTaskGizmos>,
    selected_ships: Query<(&TaskQueue, &Transform), With<IsEntitySelected>>,
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
                TaskInsideQueue::HarvestGas { target, .. } => {
                    let planet_pos = all_transforms.get(target.into()).unwrap().translation;
                    gizmos.line(current_position, planet_pos, GIZMO_COLOR);
                }
                TaskInsideQueue::AwaitingSignal { .. } => {}
                TaskInsideQueue::RequestAccess { .. } => {}
                TaskInsideQueue::DockAtEntity { target } => {
                    let target_position = all_transforms.get(target.into()).unwrap().translation;
                    gizmos.line(current_position, target_position, GIZMO_COLOR);
                    current_position = target_position;
                }
                TaskInsideQueue::Undock => {}
                TaskInsideQueue::Construct { target } => {
                    // Task target might become invalid during the frame where construction is finished
                    if let Ok(target_transform) = all_transforms.get(target.into()) {
                        gizmos.line(current_position, target_transform.translation, GIZMO_COLOR);
                    }
                }
            }
        }
    }
}
