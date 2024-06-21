Too lazy to manage a whole kanban board and issues for these things yet. Roughly sorted, these should be the next steps:

# Improved Trading

- Add (more) Items
- Ship AI decides which items to pick up and sell
- Stations reserve goods & storage space for incoming trades

# Multiple Stations

- Stations can buy and sell only specific items
- Ship AI searches for the best deal

# Simple Debug UI

We'll have to implement some kind of fake physics for this. Circular colliders should be enough for pretty much
everything, and I'm scared of testing this with a million entities as long as we don't separate them into sectors. :^)

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
