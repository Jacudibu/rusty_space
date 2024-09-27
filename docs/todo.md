Too lazy to manage a whole kanban board and issues for these things yet. Roughly sorted by priority. The further down we go, the less refined these get. Recently completed stuff is ~~striked through~~ and eventually deleted during cleanups.

Current performance goal: 
1 Million Ships getting processed across the universe and 100k of them actively rendered @30 FPS on my Ryzen 7 5700X
If my machine can handle that, I'd assume a potato can run 10% of that smoothly, which is the real goal here.

(These goals will probably get lowered a little downwards as soon as combat gets implemented, but only time will tell.)

# Improved Trading

- Individual inventory capacities for each item
- Individual m³-storage consumption for each item
- Automatically generate relevant buy & sell orders for stations, depending on existing station modules

# Multiple Stations

- Ship AI can buy from one seller and plan ahead to sell to multiple buyers at once, within given range
  We iterate through all offers anyway, might just as well keep an array of the best offers and check if any of them are
  on route to the final candidate if there's still some storage capacity left.

# Less Debug Values

- ~~Probably easier with `leafwing_manifest` and maybe also `bevy_common_assets`~~ 
- ~~Add parsing for data files, remove hardcoded Items~~ (debug stuff will remain in-code, at least until we got around to make leafwing_manifest support a folder with files instead of one single big file. Can be serialized with serde later on.)
- Change items and recipes to stuff that makes sense
- Spawn one station for every production module & recipe

# Station Building

- New stations can be created in a running game
- Construction Materials go into separate inventory
- Builder ships build stations with their drones or something
- Station module costs increases with station size

Modules:

- Production (one module per item... or per recipe?)
- Storage (At this point capacity won't be hardcoded anymore, yay!)
- Docking (At this point we will need to implement a docking queue. Will look funny.)
- Ship Building (With every module supporting constructing differently sized ships)
- Defense (later on)

# Sectors
## Sector Resources

Sectors have different resource distributions, requiring either trade or expansion to fix local scarcity and rising
demands.

- Asteroid Density and yields depend on sector values. Wouldn't bother with station collisions, though extra protective measurements could be a nice excuse to make stations more expensive in these sectors.

## Planets

- ~~Gas giants serve as a reliable source of certain raw resources~~
- Some Production (mainly Energy Cell) may depend on available solar power in sector
- Solid planets can be colonized by the sector owner for additional resources, but usually it should be both cheaper and more efficient to just harvest more asteroids rather than bothering with the extra costs from dealing with various atmospheres and gravitation. However, once the entire Universe is colonized and borders are well established in between factions and resources grow sparse, they might be a way to unlock additional resource production over time.


# Task System Overhaul

- See if beet might help implementing some of the more complex behaviors: https://github.com/mrchantey/beet
- Main tasks are handed out by the ShipBehavior, and are then dynamically filled with subtasks to complete them.
    - e.g. AutoTrade: Just add `Buy X` and `Sell x`, then do the pathfinding in a more concurrent system once it becomes relevant.

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

# Upgrades & Research

Certain properties of Stations and ships should be upgradeable in small % intervals by investing a big amount of
resources.
E.g. Engine Speed or turning angle, or station storage.
