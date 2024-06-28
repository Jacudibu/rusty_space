Too lazy to manage a whole kanban board and issues for these things yet. Roughly sorted, these should be the next steps:

# Improved Trading

- ~~Add more Items~~
- ~~Sell & Buy offers should be different components so we can iterate over them at the same time~~
- ~~Ship AI decides which items to pick up and sell~~
- ~~Stations reserve goods & storage space for incoming trades... phantom inventories!~~
- ~~Dynamic Pricing~~
- Individual inventory capacities for each item
- Individual m³-storage consumption for each item

# Multiple Stations

- ~~Add more Stations~~
- ~~Stations can buy and sell only specific items~~
- ~~Ship AI searches for the best deal~~
- Ship AI can buy from one seller and plan ahead to sell to multiple buyers at once, within given range
  We iterate through all offers anyway, might just as well keep an array of the best offers and check if any of them are
  on route to the final candidate if there's still some storage capacity left.

# Simple Debug UI

- ~~Process mouse clicks~~
- ~~Highlight selected objects~~
- ~~Click on station, see storage.~~
- ~~Click on ship, see storage and task information.~~
- ~~List how many units of each individual type have been selected~~

# Production

- ~~Add item recipes~~
- ~~Stations will process one or more ingredients to one or more new items~~
- ~~Stations can have multiple production modules, with individual productions~~
- Enable optional Variable yield depending on (placeholder) Sector Settings
- ~~Shipyard Module will produce new ships~~

# Less Debug Values

- Add parsing for data files, remove hardcoded Items
- Change items and recipes to stuff that makes sense
- Spawn one station for every production module & recipe

# Task System Overhaul

Main tasks are handed out by the AI, and are then dynamically filled with the subtasks required to fulfill them.
Not sure how deeply the pathfinding results should be cached here. Depends on performance.

```
Buy 50 X
  |- Move to System
  |- Move to System
  |- Move to Station
  |- Dock
  |- Exchange Wares
```

```
Sell 50 X
  |- Undock
  |- Move to Station
  |- Dock
  |- Exchange Wares
```

# Station Building

- New stations can be created in a running game
- Construction Materials go into separate inventory
- Builder ships build stations with their drones or something
- Station module costs increases with station size

Modules:

- Production (one module per item... or per recipe?)
- Storage (At this point capacity won't be hardcoded anymore, yay!)
- Docking (At this point we will need to implement a docking queue. Will look funny.)
- Ship Building
- Defense (later on)

# Sectors

Sectors keep track of the entities inside them, allowing for localized physics and unit selection.

- Separate the map into hexagonal sectors which are connected through gates
- Ships can only travel between sectors by using gates
- Draw lines between gates
- Draw borders around sectors
- UI should display the name of the sector that's currently being hovered over

# Sector Resources

Sectors have different resource distributions, requiring either trade or expansion to fix local scarcity and rising
demands.

- Asteroids could randomly spawn at the sector edges and drift through them, until either getting harvested or leaving
  the sector again. Density and yields depend on sector values. Wouldn't bother with station collisions, though extra
  protective measurements could be a nice excuse to make stations more expensive in these sectors.
- Gas Clouds... no clue. Maybe having ships or stations suck up the atmosphere of a Gas Giant? But that would make
  limiting hourly yields fairly hard or unlogical.

# Multiplayer

Implement multiplayer with selectable "Sync Intensity" values. (It's just a state, ez)
These will limit which systems run exclusively on the host, which will then send network events to the connected
clients.

Level 1: Synchronize Task creation (Bare minimum, limiting the big AI decision-making to the host.)
Level 2: Synchronize Sector transitions (Should improve positional sync, but might not even be necessary)
Level 3: Synchronize Combat events (Hit detection / damage. This will be necessary for competitive play)
Level 4: Synchronize Ships (The true performance nightmare we want to avoid at all costs)

# Player Control

Allow giving individual tasks to ships. There should be a way to add them to the top and the end of the queue.

# Factions

Different factions may claim sectors and may or may not like each other.
Since storage space will always be reserved for each individual delivery, missed and delayed deliveries could
dynamically decrease faction standing.

- Tint each object to represent its faction color
- Respect faction relations in Task creation (don't enter hostile sectors, don't trade with hostile stations...)
- Players are factions

# Advanced Unit Selection

Units aren't completely selected until the mouse button is actually released. Add some kind of Hover Step.
Transitioning from `Hovered` to `Selected` might be a bit ugly for change detection as long as it's only managed via
components.

- Shift+Clicking should not clear previous selection, selects additional entities
- CTRL+Clicking does not clear previous selection, deselects entities

# Upgrades & Research

Certain properties of Stations and ships should be upgradeable in small % intervals by investing a big amount of
resources.
E.g. Engine Speed or turning angle, or station storage.
