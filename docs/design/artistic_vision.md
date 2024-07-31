# Art? What is art? 

Anything player-made will be displayed as abstract low res (8x8 or 16x16) sprites. White with black borders. Beautiful programmer art. Quick to produce, cheap to render.

Icons will either be sprites or vector graphics (or sprites rendered from vector graphics). We could also extract those sprites at startup in different resolutions from vector graphics depending on game performance settings. `vello` and `bevy_vello` might be interesting for either of that.

A selected object will have a rectangular edges drawn around it. This should just be a simple sprite swap - these swapped sprites can also be generated from the existing sprites, rather than be stored/created manually.

### Colors
Anything player-made will be tinted in the respective faction color. There should also be a toggleable "Me/Friends/Enemies" color setting to further simplify that color coding.

### Custom Ship Icons
Players can combine a variety of smaller icons to create their own ship icons for different roles.

### Planets
In order to differentiate between all that player-made stuff and nature, Planets and Suns could be displayed as actual 3D Spheres - that'd allow us to visualize their orbital inclination as well as their axial rotation, whilst also enabling us to use bevy's fancy lighting effects for them.

Gas giants could be multiple spheres with transparent textures, rotating individually for an extra fancy effect.
