use common::components::task_kind::TaskKind;
use common::types::ship_tasks;
use pathfinding::PathElement;
use std::collections::VecDeque;

/// Creates the individual tasks required to follow a precalculated path.
pub fn create_tasks_to_follow_path(queue: &mut VecDeque<TaskKind>, path: Vec<PathElement>) {
    for x in path {
        queue.push_back(TaskKind::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: x.gate_pair.from.into(),
                stop_at_target: false,
                desired_distance_to_target: 0.0,
            },
        });
        queue.push_back(TaskKind::UseGate {
            data: ship_tasks::UseGate::new(x.gate_pair.from, x.exit_sector),
        })
    }
}
