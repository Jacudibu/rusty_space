Multiplayer will be a big headache to get working.

### Non-Functional stuff.
**Co-Op should be the main priority.** Thus, it's perfectly fine if combat feels a bit wonky for clients. Similarly, I'm not too worried about cheating or hiding data from specific clients. Players should be able to share the same faction, or to play as separate factions.

### Roadmap to Multiplayer
First, we got to define a way on how to initially sync the game
1. Saving and Loading Games

Next up, we need to get our clients connected with each other. This can happen either in a lobby or directly during gameplay. The latter is probably a little more annoying to set up.
2. Add an option to host a game
3. Add an option to join a hosted game
   - Clients download map from host
   - Host simulation pauses during download to avoid needing to catch up.

With perfectly deterministic lockstep, this would be enough. Asteroids and Stations will probably run fine forever without much intervention. However, ships and task creation probably is going to struggle due to floating point and multithreading headaches.

4. Experiment
   - Move Simulation logic into fixed timestep 
     - Having all client logic run at the same UPS value probably helps decrease timing issues
     - We could use a 2D `SimulatedTransform` Component and just interpolate all Transforms in `Update`. This also saves us all the 3D truncation headaches. See https://bevy-cheatbook.github.io/cookbook/smooth-movement.html
   - Task Creation is probably going to be the biggest headache.
     - Let Task Creation run on every client, but ensure entities are sorted
     - Only let task creation run on host, sync results to clients
   - Periodically sync units, depending on available network bandwidth
   - If a client detects something might be off with a specific entity, request immediate sync
   - If a client selects an entity, immediately sync it

We could use bevy_replicon, lightyear, or just bevy_simplenet... technically we don't need any of the automated ECS replication features, just sending events more or less reliably over network would already be enough, meaning we'd entirely skip their respective `Replicated` marker components.



### Interesting Reads
Gaffer On Games about Floating points and determinism (spoiler: they are non-deterministic)<br> 
https://gafferongames.com/post/floating_point_determinism/

AI War I actually used Lockstep, whereas AI War II uses a healthy mix of everything to self-heal from desyncs whenever they happen. Very inspirational. Game logic apparently runs at just 10 UPS.<br>
https://wiki.arcengames.com/index.php?title=Category:AI_War_2:_All_About_Multiplayer#AI_War_Classic:_The_Lockstep_Model

Factorios first Version supporting Multiplayer was 0.11 in late 2014. It's using Lockstep.<br>
First blog post ever talking about the topic: https://www.factorio.com/blog/post/fff-76<br>
There are many more where they talk about all their issues with desync.





