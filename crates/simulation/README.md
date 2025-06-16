# simulation
This is where the entire universe gets simulated. Every module exports its own plugin, which then gets bundled up together and re-exported again.

(Technically, all of those could be their own creates. Maybe something for the future?)

### asteroids
Updates Asteroids: Movement, (re-/de-)spawning and animating them.

### physics
Updates orbits and provides some 2d collision methods.

### production
Provides systems which handle material and ship production systems.