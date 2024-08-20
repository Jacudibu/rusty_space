# Ship Configuration

Ships can be assembled from a multitude of different tiles and thus be specialized to do one specific thing very well.

# Ship Configuration in Other Games
## EVE Online
Limited by CPU (tf) + Capacitor (GJ)

Slots:
- High - Weapons, Drone Boosters, EWAR, Resource Harvesting
- Medium - Mostly "Active" Modules which need to be toggled on or off. Boost repairs, propulsion, defenses etc.
- Low Power - Passive Modules which just boost all kinds of ship stats. 
- Rigs - Boost certain stats with no CPU/Cap cost, instead each slot uses a shared amount of calibration points.


- Different Damage Types: EM, Kinetic, Thermal, Explosive
- Different HP Types: Shield, Armor, Hull

## X4
No Limits

Every Ship has a hardcoded number of slots for
- Front-Facing Weapons 
- Turrets
- Shields
- Engines
- Thrusters
- Software (bad idea)
- Crew (bad idea)


- Different HP Types: Shields, Hull

which can be filled more or (for borons) less freely with modules. There's a dozen different weapon categories, but they all seem to deal the same kind of damage (with some exceptions that feel fairly unbalanced) and only vary in range, spread, fire rate and damage per hit


# --> We have

Damage Types: 
- Energy (Lasers. High Energy, Low CPU. Strong against Shields.)
- Impact (Bullets, Medium Energy, Medium CPU. Strong against Hulls. How deep do we want to go into ammo?)
- Explosive (Missiles, Low Energy, High CPU. Strong against both, but requires the correct missile types. Less damage to fast moving targets.)

Health Types: Shield + Armor, maybe Hull?

## Chassis 
Defines general specs of the ships and limits what can be done with it by only providing a certain number of slots for turrets / engines.

Specs:
- Power & CPU Capacity
- HP & Shield
- Maneuverability (The bigger the harder)
- High Power Slots (For weapons and some strong, passive boosts)
- Utility Slots (Mix of EVE Medium and Low power utilities)
- Shield Config (Slider: Capacity <--> Recharge. Or let the player pick between three variants)
- Engine Config (Slider: Turning <--> Max Speed. Or let the player pick between three variants. Combined X4 Engine + Thruster.)
- Mod Slots (Optional Late Game stuff. Basically EVE Rigging slots but with a fancier name)

## Slots

### High Power
#### Mining Laser
Only ships equipped with mining lasers can mine asteroids. Those are too weak to deal any significant damage to properly armored or shielded ships, but perfect for cutting defenseless rocks floating through space.

#### Gas Collector
Functioning like a vacuum cleaner, only ships equipped with a gas collector can harvest from gas giants.

#### TODO: Weapons
Pew Pew!

### Utility
#### Cargo Bays
Increases the amount of cargo this ship can carry at the cost of maneuverability.

#### Shield Generator
Increases available shield capacity at the cost of energy.

#### Hull Plating
Increases available armor, at the cost of maneuverability.

#### Engine Link
Increases Turning Angles at the cost of CPU.

#### Engine Boost
Increases Speed and Acceleration at the cost of Energy.

#### Turret Mounts
Increases the arc at which a ship can attack its targets, at the cost of CPU and max range.

#### Drone Bay
Increases the amount of drones this ship can launch. Costs Power and CPU.



# In-Game Configurator
- Ship Icon can be freely set (and combined with small icons) by the player
- Ships can be upgraded to different configurations.

I'm envisioning a fancy editor window split into three panels
- Left Side: Configuration. 
- Upper Right Side: Ship with gizmos depicting its firing arc and range.
- Lower Right Side: Fancy graphs depicting all the ship stats.
