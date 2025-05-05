# persistence
This crate handles writing, loading and migrating save files.

`data` contains a complete copy of all relevant components and resources which need to be persisted. 
Breaking changes in the data structure require a new version of the affected struct to be created, which will allow us to migrate savegames from older versions to the latest format.

Only the latest version will be available outside of this crate, through `persistance::data`.

Right now, only loading is implemented and mainly used in combination with tests and mock instances. 