Currently doing a deep dive into the inner workings of bevy and possible alternatives. Collecting ideas on how to
optimize things for our use case in here in case I end up using bevy in some capacity in the end.

# Sectors

Sectors store an array of EntityIDs of ships inside them. Instead of updating all ships at once, we'll only process a
bunch of sectors per frame. Ships will be removed from a sector as soon as they enter the gate.

A resource will keep track of all ships which are currently transitioning, and they'll get an extra component to be
ignored by other systems.

# Balancing

In X4, players can usually just pick a single sector as their home base and put an infinite amount of infinitely big
Stations there. While practical (and also rather logical), this approach completely removes any need to actually expand.
In order to remedy this and make sector ownership matter more, building bigger stations or multiple stations in the same
sector should come at a cost.
Some ideas:

- Systems with lots and big stations attract space bugs, which want to eat that juicy metal (Similar to Kha'ak in X4,
  but instead of just randomly appearing in mining areas, they get drawn to the stations here)
- Station module costs increase for each module already added to the station
- Stations themselves become more expensive the more there already are in the system. Could explain that away with
  requiring some hard-to-craft devices to withstand weird space magic effects which gets harder the more there already
  are in the vicinity or so.

# Rendering

Might try to spin our own there, allowing us the following:

- Using proper 2D Transforms, dropping the Z values for vectors and simplifying rotations.
- Every sector could use `[0,0]` as the global origin, decreasing float imprecision within the simulation.

But before we invest any energy into this, we first need to have a bunch of cross-sector logic going and the whole
simulation needs to be fleshed out enough to do some meaningful benchmarks. A nice learning opportunity, even if things
might fail.
