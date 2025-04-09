Attempts to figure out the perfect inventory system

# Glossary
- `Allowed Items` refers to which items may fit into the respective inventory


# Areas
## Ships
All Items allowed at any time, though behaviours focus on one at a time.
Storage Space limited by ship configuration, never changes once built. (unless reconfigured)
Dynamic Buy, Sell what isn't needed. (capital ships only). Prices should default to Demand-Based + X to expedite resupply.
~~Special Storage Types for certain, hard to transport goods (Ore, Gas)~~ KISS

## Stations
Allowed Items dictated by production
Storage Space limited by constructed modules, changes dynamically. Maybe with player intervention?
Dynamic Buy + Sell based on inventory capacities, Prices depend on inventory.
~~Special Storage Types for certain, hard to transport goods (Ore, Gas)~~ KISS

## Construction Sites
Allowed Items dictated by materials
Storage space basically infinite (up to however many goods are required)
Static Buy + Sell: Only buy goods we need in exact quantities, sell everything we don't. Prices default to Max/Min.