use bevy::app::{App, Startup, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    AppGizmoBuilder, GizmoConfigGroup, GizmoConfigStore, Gizmos, Plugin, Query, Reflect, ResMut,
    Srgba, Transform, With,
};
use common::components::Gate;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use entity_selection::components::IsEntitySelected;

pub struct ShipTaskGizmoPlugin;
impl Plugin for ShipTaskGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<SelectedShipTaskGizmos>();
        app.add_systems(Startup, configure);
        app.add_systems(Update, draw_selected_ship_task);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct SelectedShipTaskGizmos;

const GIZMO_COLOR: Srgba = bevy::color::palettes::css::CORNFLOWER_BLUE;

fn configure(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<SelectedShipTaskGizmos>();
    config.line.width = 4.0;
}

fn draw_selected_ship_task(
    mut gizmos: Gizmos<SelectedShipTaskGizmos>,
    selected_ships: Query<(&TaskQueue, &Transform), With<IsEntitySelected>>,
    all_transforms: Query<&Transform>,
    all_gates: Query<&Gate>,
) {
    for (task_queue, transform) in selected_ships.iter() {
        let Some(active_task) = &task_queue.active_task else {
            continue;
        };

        let mut current_position = draw_gizmos_for_task(
            &mut gizmos,
            all_transforms,
            all_gates,
            transform.translation,
            active_task,
        );

        for x in &task_queue.queue {
            current_position =
                draw_gizmos_for_task(&mut gizmos, all_transforms, all_gates, current_position, x);
        }
    }
}

/// Draws gizmos for the given task and returns the position the ship will have once the task is finished.
fn draw_gizmos_for_task(
    gizmos: &mut Gizmos<SelectedShipTaskGizmos>,
    all_transforms: Query<&Transform>,
    all_gates: Query<&Gate>,
    current_position: Vec3,
    task: &TaskKind,
) -> Vec3 {
    match task {
        TaskKind::ExchangeWares { .. } => current_position,
        TaskKind::MoveToEntity { data } => {
            let target_position = all_transforms.get(data.target.into()).unwrap().translation;
            gizmos.line(current_position, target_position, GIZMO_COLOR);
            target_position
        }
        TaskKind::UseGate { data } => {
            let gate = all_gates.get(data.enter_gate.into()).unwrap();
            gizmos.linestrip_2d(gate.transit_curve.iter_positions(10), GIZMO_COLOR);
            gate.transit_curve.position(1.0).extend(0.0)
        }
        TaskKind::MineAsteroid { data } => {
            let asteroid_pos = all_transforms.get(data.target.into()).unwrap().translation;
            gizmos.line(current_position, asteroid_pos, GIZMO_COLOR);
            current_position
        }
        TaskKind::HarvestGas { data } => {
            let planet_pos = all_transforms.get(data.target.into()).unwrap().translation;
            gizmos.line(current_position, planet_pos, GIZMO_COLOR);
            current_position
        }
        TaskKind::AwaitingSignal { .. } => current_position,
        TaskKind::RequestAccess { .. } => current_position,
        TaskKind::DockAtEntity { data } => {
            let target_position = all_transforms.get(data.target.into()).unwrap().translation;
            gizmos.line(current_position, target_position, GIZMO_COLOR);
            target_position
        }
        TaskKind::Undock { .. } => current_position,
        TaskKind::Construct { data } => {
            // Task target might become invalid during the frame where construction is finished
            if let Ok(target_transform) = all_transforms.get(data.target.into()) {
                gizmos.line(current_position, target_transform.translation, GIZMO_COLOR);
            }
            current_position
        }
    }
}
