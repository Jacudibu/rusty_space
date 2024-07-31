Multiplayer will be a big headache to get working.

### Non-Functional stuff.
If networking ends up getting implemented, **Co-Op should be the main priority.** 
Thus, it's perfectly fine if combat feels a bit wonky for clients. Similarly, I'm not too worried about cheating or hiding data from specific clients. Players should be able to share the same faction, or to play as separate factions. 

Code Adjustments necessary for networking should only have a minimal impact on single-player performance.

### Roadmap to Multiplayer
First, we got to define a way on how to initially sync the game
- [x] Saving and Loading Game
   - This includes assigning unique IDs to every relevant entity which are valid across sessions

Next up, we need to get our clients connected with each other. This can happen either in a lobby or directly during gameplay. The latter is probably a little more annoying to set up, but very nice to have.
- [ ] Add an option to host a game 
- [ ] Add an option to join a hosted game
   - Clients download map from host
   - Host simulation pauses during download to avoid needing to catch up.

With perfectly deterministic lockstep, this would be enough. Asteroids and Stations will probably run fine forever without much intervention. However, ships and task creation probably is going to struggle due to floating point and multithreading headaches.

### Experiment
- Change Simulation Logic to only use fixed floating points. Either via the [`fixed` crate](https://crates.io/crates/fixed) or by just scaling local coordinates by factor 100000 and storing them as i32 or so.
  - Avoids all the terrible things about floating point imprecision
  - Might be a lot of work to get right
- Task Creation is probably going to be the biggest headache.
  1) Let Task Creation run on every client, but ensure entities are sorted
  2) Only let task creation run on host, sync results to clients
- Periodically sync units (one by one, never all at once), depending on available network bandwidth
- If a client detects something might be off with a specific entity based on incoming commands, request immediate sync
- If a client selects an entity, immediately sync it


### External Crates
We could use `bevy_replicon`, `lightyear`, or just `bevy_simplenet`... technically we don't need any of the automated ECS replication features, just sending events more or less reliably over network would already be enough, meaning we'd entirely skip their respective `Replicated` marker components.



### Interesting Reads
Gaffer On Games about Floating points and determinism (spoiler: they are non-deterministic and ugly)<br> 
https://gafferongames.com/post/floating_point_determinism/

AI War I actually used Lockstep, whereas AI War II uses a healthy mix of everything to self-heal from desyncs whenever they happen. Very inspirational. Game logic apparently runs at just 10 UPS.<br>
https://wiki.arcengames.com/index.php?title=Category:AI_War_2:_All_About_Multiplayer#AI_War_Classic:_The_Lockstep_Model

Factorios first Version supporting Multiplayer was 0.11 in late 2014. It's using Lockstep.<br>
First blog post ever talking about the topic: https://www.factorio.com/blog/post/fff-76<br>
There are many more where they talk about all their issues with desync.





