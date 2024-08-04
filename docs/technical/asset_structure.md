All game assets are stored in `/assets`, and further grouped into `AssetPacks` by their respective subfolders. All base game content is kept within the `base` AssetPack. Overall, our folder structure will look like this:

```
 assets
  |- base
  |   |- i18n
  |   |- items
  |   |- recipes
  |   |- [...]
  |- dlc_1
  |   |- i18n
  |   |- items
  |   |- recipes
  |   |- [...]
  |- mod_a
  |- mod_b
  |- [...]
```

The first level of subfolders within `/assets` divides assets by origin: each dlc and mod gets their own subfolder.

- Each asset has a unique id, represented as a String: `id: "item_a"`
- Assets can refer to each other through that id field: `produces: "item_a"`
- Assets may refer to assets from other asset packs by prefixing: `produces: "base:item_a"` or `refers_to: mod_a:item_b`
- Localized names and descriptions are stored in a separate `i18n` or `locale` file/folder. (Which is not relevant until we have an actual GUI.)

### Data Overrides
Some AssetPacks might want to change existing data rather than adding new one: Adding more translations to items or minor value tweaks... All of that should be possible, somehow.

### AssetPack manifest file 
Every Asset Pack contains a file holding all important information regarding it, for further use in a mod loader and stuff like that.

```json
{
  "id": "mod_a",
  "version": "1.2.3",
  "title": "My Fancy Mod",
  "authors": ["A Special Person"],
  "dependencies": ["mod_a"]
}
```
