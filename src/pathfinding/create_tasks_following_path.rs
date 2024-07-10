use crate::pathfinding::PathElement;
use crate::ship_ai::{TaskInsideQueue, TaskQueue};

pub fn create_tasks_to_follow_path(queue: &mut TaskQueue, path: Vec<PathElement>) {
    for x in path {
        queue.push_back(TaskInsideQueue::MoveToEntity {
            target: x.gate_pair.from.into(),
            stop_at_target: false,
        });
        queue.push_back(TaskInsideQueue::UseGate {
            enter_gate: x.gate_pair.from,
            exit_sector: x.exit_sector,
        })
    }
}
