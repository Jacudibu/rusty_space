# simulation
This is where the entire universe gets simulated. Every module exports its own plugin, which then gets bundled up together and re-exported again.

(Technically, all of those could be their own creates. Maybe something for the future?)

### asteroids
Updates Asteroids: Movement, (re-/de-)spawning and animating them.

### physics
Updates orbits and provides some 2d collision methods.

### production
Provides systems which handle material and ship production systems.

### ship_ai
Every ship has a behavior attached to it, which in turn selects the tasks the ship should execute.

In multiplayer sessions, behaviors are only run on the session owner's machine, and the task lists get synchronized across clients... which hopefully is enough to have some sort of pseudo-lockstep with minimal automated desync correction.

#### Adding new ShipTasks
1. Add a `NewTask` struct in (common) `ship_tasks`
2. Add it to (common) `TaskKind`
3. Add `Without<ShipTask<NewTask>>` to `ShipIsIdleFilter`
4. Implement all `task_lifecycle_traits` for `NewTask`
5. Call `register_task_lifecycle` in `/ship_ai/plugin.rs`