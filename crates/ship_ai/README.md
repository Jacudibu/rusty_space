# ship_ai
Every ship has a `ShipBehavior<T>` attached to it, which in turn selects the tasks the ship should execute.
Tasks are then filled into the `TaskQueue`, and the active task will be attached to the entity as `ShipTask<T>`.

In multiplayer sessions, behaviors are only run on the session owner's machine, and the task lists get synchronized across clients... which hopefully is enough to have some sort of pseudo-lockstep with minimal automated desync correction.

#### Adding new ShipTasks
1. Add a `NewTask` struct in (common) `ship_tasks`
2. Add it to (common) `TaskKind`
3. Fix compile errors for missing `TaskKind` matches
4. Implement all `task_lifecycle_traits` for `NewTask`
5. Implement `TaskMetaData` for `NewTask`