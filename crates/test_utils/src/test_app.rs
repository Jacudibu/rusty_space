use bevy::app::TaskPoolPlugin;
use bevy::asset::{AssetApp, AssetPlugin};
use bevy::image::Image;
use bevy::prelude::App;

/// Helps us to quickly build a barebones bevy [App] within tests.
pub struct TestApp {}

impl TestApp {
    pub fn with_stations() {}
    pub fn with_sectors() {}
    pub fn with_ships() {}
    pub fn with_gate_pairs() {}

    pub fn build(self) -> App {
        let mut app = App::new();

        // app.init_resource::<MapLayout>();
        // app.init_resource::<SpriteHandles>();
        // app.init_resource::<PrecomputedOrbitDirections>();

        // all of these are required to get GameData::from_world working
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Image>();

        // GameData::initialize_mock_data(app.world_mut());
        // SessionData::initialize_mock_data(app.world_mut());
        //
        // app.insert_resource(self.sectors);
        // app.insert_resource(self.gate_pairs);
        // app.insert_resource(self.stations);
        // app.insert_resource(self.ships);
        //
        // app.add_plugins(UniverseSaveDataLoadingOnStartupPlugin);

        app.finish();
        app.update();
        app
    }
}
