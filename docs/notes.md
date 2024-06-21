Currently doing a deep dive into the inner workings of bevy and possible alternatives. Collecting ideas on how to
optimize things for our use case in here in case I end up using bevy in some capacity in the end.

# Sectors

Sectors store an array of EntityIDs of ships inside them. Instead of updating all ships at once, we'll only process a
bunch of sectors per frame. Ships will be removed from a sector as soon as they enter the gate.

A resource will keep track of all ships which are currently transitioning, and they'll get an extra component to be
ignored by other systems.

# Rendering

Might try to spin our own there, allowing us the following:

- Using proper 2D Transforms, dropping the Z values for vectors and simplifying rotations.
- Every sector could use `[0,0]` as the global origin, decreasing float imprecision within the simulation.

But before we invest any energy into this, we first need to have a bunch of cross-sector logic going and the whole
simulation needs to be fleshed out enough to do some meaningful benchmarks. A nice learning opportunity, even if things
might fail.
