Too lazy to manage a whole kanban board and issues for these things yet. Roughly sorted by priority. The further down we go, the less refined these get. Recently completed stuff is ~~striked through~~ and eventually deleted during cleanups.

> Current performance goal: 
> 1 Million Ships getting processed across the universe and 100k of them actively rendered @30 FPS on my Ryzen 7 5700X
> If my machine can handle that, I'd assume a potato can run 10% of that smoothly, which is the real goal here.
> 
> (These goals might change as soon as combat gets implemented, but until then, this here is the plan.)

# Task Cancellation
~~Now that unexpected things can happen, every Task needs to be cancellable, both by UI and systems (and cancellation through systems should probably count as a bug, but is still good to avoid crashes).~~
~~Additionally, add a cancel button to GUI.~~
Maybe entities should keep track of entities having tasks assigned to them to notify all dependent entities for task cancellations in case things get despawned?

# Testing
We need to figure out a proper testing strategy to support the simulation as it grows.

# Improved Inventories
~~- Individual inventory capacities for each item~~
~~- Figure out if we want to separate weight and volume, or if this is just an unnecessary complication... (hint to future-me: it probably is)~~
- Unit tests for `InventoryComponent`
- Remove reservation for items in production lines, just delay completion in case there's not enough room.

# Automated Trade Order System
- Automatically generate relevant buy and sell orders for stations, depending on existing station modules. Replaces more hardcoded stuff.

# Better Camera Controls 
- Zoom to MouseCursor
- Optional Edge Panning
- CameraPosition Hotkeys (save & jump to camera position X by pressing some key combination)

# Station Building

- ~~Build-Site Entities~~
- ~~Construction Materials go into separate inventory~~
- ~~Builder ships build stations (builders being near the construction site should be enough to apply their constant construction strength)~~
- ~~Multiple builders speed up construction (maybe not completely linearly, so players might want to consider bigger construction ships over a swarm of cheap small ones)~~
  - ~~Builders just register themselves to a construction site, and the construction site itself actually increments its progress each tick~~
- Construction Sites are properly loaded from save files
  - Persist Buy/Sell Order data 
- Station module costs increase with station size

- ~~New stations can be created in a running game~~
  - ~~Pressing C and left-clicking somewhere should just place a new construction site as a PoC~~

Modules:

- Production
- Storage (At this point capacities won't be hardcoded anymore, yay!)
- Docking (At this point we will need to implement a docking queue. Will look funny.)
- Ship Building (With every module supporting constructing differently sized ships, maybe with speed modifiers for some)
- Defense (later on)
- Sector Claim

# Future Proofing
- Destroy ships when pressing DEL
  - Verifies task Cancellation is properly implemented
  - ...but also write tests for that :>

# Sectors
## Sector Resources

Sectors have different resource distributions, requiring either trade or expansion to fix local scarcity and rising
demands.

- Asteroid Density and yields depend on sector values. Wouldn't bother with station collisions, though extra protective measurements could be a nice excuse to make stations more expensive in these sectors, maybe including some running costs too. On the other hand side, a station equipped with an anti-asteroid weapon could passively harvest asteroids?

# Multiple Stations

- Ship AI can buy from one seller and plan ahead to sell to multiple buyers at once, within given range
  We iterate through all offers anyway, might just as well keep an array of the best offers and check if any of them are
  on route to the final candidate if there's still some storage capacity left.

# Randomized Universe Generation
- Add somewhat random universe generation during the next PROCJAM (probably happening in december?)

## Planets

- ~~Gas giants serve as a reliable source of certain raw resources~~
- Some Production (mainly Energy Cell) may depend on available solar power in sector
- Solid planets can be colonized by the sector owner for additional resources, but usually it should be both cheaper and more efficient to just harvest more asteroids rather than bothering with the extra costs from dealing with various atmospheres and gravitation. However, once the entire Universe is colonized and borders are well established in between factions and resources grow sparse, they might be a way to unlock additional resource production over time.

# Task System Overhaul

- See if `beet` might help implementing some of the more complex behaviors: https://github.com/mrchantey/beet
- Main tasks are handed out by the ShipBehavior, and are then dynamically filled with subtasks to complete them.
    - e.g. AutoTrade: Just add `Buy X` and `Sell x`, then do the pathfinding in a more concurrent system once it becomes relevant.
- Ship Behavior idle ship filter could be done in a par_iter_mut system for all ships by moving the timestamp into a separate component and adding/removing an "BehaviorUpdateRequested" kinda marker component. Would need profiling to see if it's actually better that way.

### Examples

#### AutoTrade

```
Search for good deals (repeats every couple seconds if nothing was found)

Buy 50 X
  |- Move to System
  |- Move to System
  |- Move to Station
  |- Dock
  |- Exchange Wares

Sell 50 X
  |- Undock
  |- Move to Station
  |- Dock
  |- Exchange Wares
  
(Repeat)
```

#### AutoBuild:

```
Search for construction sites lacking builders (repeats every couple seconds if nothing was found)

Build Station
  |- Move to System
  |- Move to Station
  |- Build

```

#### Sector Patrol

Probably not 100% accurate given that combat isn't implemented yet

```
Search for hostile enemies (repeated every couple seconds if nothing was found)

Attack Target 
  |- Fly to target
  |- Attack target

(Repeat)
```

# Persistence

- Saving UniverseSaveData to file
- Saving occurs in the background, without interrupting gameplay (data collected similar to render extract phase, then yeeted into separate thread for background write operation)
- Loading UniverseSaveData from file

# Multiplayer

Adding multiplayer to this already is and also will be an ongoing adventure.
It's further defined [here](technical/networking.md)

# Player Control

Allow giving individual tasks to ships. There should be a way to add them to the top and the end of the queue.
- Right-clicking in space should add a MoveTo Task to selected ships
- Right-clicking an object should... do contextually useful things.
- Unit controls should work like in Beyond All Reason - draw a line and units position themselves in a line. 

# Factions

Different factions may claim sectors and may or may not like each other.
Since storage space will always be reserved for each individual delivery, missed and delayed deliveries could
dynamically decrease faction standing.

- Tint each object to represent its faction color
- Respect faction relations in Task creation (don't enter hostile sectors, don't trade with hostile stations...)
- Players are factions

# Faction AI

Time to get cooking! Factions attempt to expand their territory by building both stations and ships on their own.
Good luck implementing all of that, future me! :>

# Advanced Unit Selection

Units aren't completely selected until the mouse button is actually released. Add some kind of Hover Step.
Transitioning from `Hovered` to `Selected` might be a bit ugly for change detection as long as it's only managed via
components.

- Shift+Clicking should not clear previous selection, selects additional entities
- CTRL+Clicking does not clear previous selection, deselects entities

# Combat

At this point the simulation should be big enough to decide how combat should work in regard to performance.
Simulating individual projectiles, beams or missiles would be cool as hecc, but the physics behind that would probably eat our CPU cores alive. Therefore, this should be a 2 Step process.

Step 1: Ship Firing Arcs
Define the firing arcs and ranges at which ships can fire, and the required physics to test if any overlaps happen. Make damage happen instantly on weapon cooldown.

Step 2: Projectiles
For the lulz, don't apply the damage instantly and spawn the projectiles. If we restrict collision detection to stuff within the same sector and have low fire rates, this *might* be manageable for a dozen or so ships. Or not. Either way, it's gonna be fun to watch for a bit. If the simulation for some strange reason still runs on a somewhat bearable framerate during two big clashes, it could be toggled on or off through a setting.

This could (and probably should) be "visual eye candy only" - the damage is still calculated instantly, but if the player is zoomed in enough, they can see fake shooty things.

# GUI Tasks
These tasks will only become relevant once we decide it's time to add a proper UI to the game.

## Manual Trade Order System
- Players can manually set up trade orders and inventory limits for their stations.

## Upgrades & Research

Certain properties of Stations and ships should be upgradeable in small % intervals by investing a big amount of resources.
E.g. Engine Speed or turning angle, or station storage.
