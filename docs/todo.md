Too lazy to manage a whole kanban board and issues for these things yet. Roughly sorted, these should be the next steps:

# Improved Trading

- Add more Items
- ~~Sell & Buy offers should be different components so we can iterate over them at the same time~~
- ~~Ship AI decides which items to pick up and sell~~
- Stations reserve goods & storage space for incoming trades... phantom inventories!
- ~~Dynamic Pricing~~

# Multiple Stations

- Add more Stations
- ~~Stations can buy and sell only specific items~~
- ~~Ship AI searches for the best deal~~
- Ship AI can buy from one seller and plan ahead to sell to multiple buyers at once, within given range
  We iterate through all offers anyway, might just as well keep an array of the best offers and check if any of them are
  on route to the final candidate if there's still some storage capacity left.

# Simple Debug UI

- ~~Process mouse clicks~~
- ~~Highlight selected objects~~
- Click on station, see storage.
- Click on ship, see storage and task information. Maybe with an option to cancel the current task to cause some chaos.

# Production

- Stations will process one or more ingredients to a new item.

# Sectors

- Separate the map into hexagonal sectors which are connected through gates
- Ships can only travel between sectors by using gates
- Draw lines between gates
- Draw borders around sectors
- UI should display the name of the sector that's currently being hovered over

# Multiplayer

Just synchronizing task creation and sector transitions should be enough to allow multiplayer to work on a cooperative
level. If we ever add combat, the required physics could be simulated just on the host's machine, which then sends
damage events over the network.

# Factions

Different factions may claim sectors and may or may not like each other.
Since storage space will always be reserved for each individual delivery, missed and delayed deliveries could
dynamically decrease faction standing.
