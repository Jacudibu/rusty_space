# Associated Structs
## ShipBehavior
They assign new TaskGroups to ship whenever they are idle. Basically a finite state machine.
Added to ship entities as `ShipBehavior<BehaviorData>`.
A ship may only have one active behavior at once.
Two special cases would be `ShipBehavior<FleetControlled>` and `ShipBehavior<StationControlled>` for ships which don't act autonomously: here the tasks will be created by the fleet or station controller in order to better coordinate actions. 

## TaskQueue
A queue of TaskGroups.
Added to ship entities as `TaskQueue` Component.
Contains an active `Option<TaskGroup>` and a `VecDequeue<TaskGroup>` to add more.
Notifies TaskGroups inside them in case of changes to the planned schedule, so they get a chance to re-evaluate their stuff. Might just update one TaskGroup per frame, front to back.

**Struct Members:**
- `active_task` : `Option<TaskGroup>` 
- `queue` : `VecDequeue<TaskGroup>` 

## TaskGroup (New!)
A collection of tasks necessary to achieve a specific goal. 
These are visible (and editable) in the GUI. (and maybe editable through gizmos at the target position when a ship is selected)
Populated with a `Vec<Task>` once they are created, which gets updated when necessary.
Can be cancelled individually. Follow-up TaskGroup(s) need to be re-evaluated accordingly.
Re-Evaluation requires the current ship position or the position where the ship ahead of this TaskGroup is expected to end up.

**Struct Members:**
- `goal` -> `TaskKind` of the goal of this TaskGroup.
- `tasks_to_achieve_goal` -> `Vec<Task>` necessary to achieve the goal. If Empty, Goal is next!
- `position_when_completed` -> Enum: `AbsolutePosition` | `StaticEntityPosition` | `Entity`
  - StaticEntityPosition only needs to check for destruction and cancel itself accordingly 
  - In case of Entity, we need to track & re-evaluate when the target entity moves sectors. That's a bit more annoying, but necessary for pretty much any task which can target ships.
  - Only really necessary to be updated when ship is selected in GUI, as ShipAIs only search for new tasks on idle ships.
- `repeat`: If true, on completion this TaskGroup is added back at the end of the TaskQueue.
- `id`: A unique ID (only needs to be unique to the assigned TaskQueue, but we could also spawn TaskGroups as entities and use that as ID?)
- `depends_on`: id of another TaskGroup which *has* to be executed before this one. Usually just needed for trade runs. 

When a ship is selected, we might want to visualize each TaskGroup as tiny, interactable gizmos; e.g. the MoveTo TaskGroup could have a drag+droppable endpoint. This would require an entity.

## Task
An individual unit of work. The current task is added to ships entities as `ShipTask<TaskData>` and run in parallel with `par_iter_mut`.
These aren't visible to the user (besides the current task as an icon), but are used to render the preview gizmo lines on selected ships.

Struct Members vary depending on the Task.

## TaskKind
Enum containing variants (containing data) for all tasks, used anywhere where we don't want to handle generics.

# Events & Task Lifecycle
- `InsertTaskIntoQueueCommand<TaskData>`
Task Creation **always** happens through. 
These make sure the TaskQueues are filled properly, and affected entities are informed about incoming ships (when applicable).

- `TaskCancellationWhileActiveRequest`
Sent when a running Task needs to be aborted.
- `TaskCancellationWhileInQueueRequest`
Sent when a task that's queued up needs to be cancelled.

- `TaskStartedEvent<TaskData>`
A task has been started.
- `TaskCompletedEvent<TaskData>`
A task has been completed.
- `TaskCanceledWhileInQueueEvent<TaskData>`
A task **inside the Queue** was cancelled. Rarely needs special handling.
- `TaskCanceledWhileActiveEvent<TaskData>`
An **active** task was cancelled. Rarely needs special handling usually the same treatment as if it was canceled whilst being active.
- `TaskMovedBackIntoQueueEvent<TaskData>`
Sent when another task with a higher priority was added to the queue, such as fleeing from attackers.

Active Tasks are added to ship Entities through a `TaskComponent<TaskData>`. There's an individual system to update the ships for each task, usually utilizing `par_iter_mut` in some way.