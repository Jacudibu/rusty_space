## ShipBehavior
They assign new TaskGroups to ship whenever they are idle.
Added to ship entities as `ShipBehavior<BehaviorData>`.
A ship may only have one active behavior. (And it should maybe be persisted inside the TaskQueue as well, so we can interact with it inside the GUI / AI)

## TaskQueue
A queue of TaskGroups.
Added to ship entities as `TaskQueue` Component.
Contains an active `Option<TaskGroup>` and a queue for more.
Notifies TaskGroups inside them in case of changes to the planned schedule, so they get a chance to re-evaluate their stuff. Might just update one TaskGroup per frame, front to back.

## TaskGroup (The missing link!)
A collection of tasks necessary to achieve a specific goal. 
These are visible (and editable) in the GUI. (and maybe editable through gizmos at the target position when a ship is selected)
Populated with Tasks once they are created, which are updated when necessary.
Can be cancelled individually. Follow-up TaskGroup(s) need to be re-evaluated accordingly.
Re-Evaluation requires the current ship position or the position where the ship ahead of this TaskGroup is expected to end up.

MetaData: 
- PositionWhenCompleted -> Enum: AbsolutePosition | StaticEntityPosition | Entity
  - StaticEntityPosition only needs to check for destruction and cancel itself accordingly 
  - In case of Entity, we need to track & re-evaluate when the target entity moves sectors.
- Repeat: If true, on completion this TaskGroup is added back at the end of the TaskQueue.

...now I wonder whether TaskGroups should be entities, that'd make updating them surprisingly easy through relationships...

## Task
An individual unit of work. The current task is added to ships entities as `ShipTask<TaskData>`
These aren't visible in the GUI, but are used to render the preview gizmo lines on selected ships.