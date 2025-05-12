use common::components::task_queue::TaskInsideQueue;
use common::components::task_queue::TaskQueue;
use common::types::ship_tasks;
use pathfinding::PathElement;

/// Creates the individual tasks required to follow a precalculated path.
pub fn create_tasks_to_follow_path(queue: &mut TaskQueue, path: Vec<PathElement>) {
    for x in path {
        queue.push_back(TaskInsideQueue::MoveToEntity {
            data: ship_tasks::MoveToEntity {
                target: x.gate_pair.from.into(),
                stop_at_target: false,
                desired_distance_to_target: 0.0,
            },
        });
        queue.push_back(TaskInsideQueue::UseGate {
            data: ship_tasks::UseGate::new(x.gate_pair.from, x.exit_sector),
        })
    }
}
