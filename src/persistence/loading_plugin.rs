use crate::initialize_data;
use crate::persistence::test_universe;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoScheduleConfigs;
use universe_builder::{gate_builder, sector_builder, ship_builder, station_builder};

pub struct UniverseSaveDataLoadingOnStartupPlugin;

// TODO: Instead of doing this all at once (and chained), separate loading into multiple States and
//       only load X entities per frame, so the app remains responsive and we can have a fancy
//       looking loading bar... plus we can then invoke it multiple times per run, yay!
impl Plugin for UniverseSaveDataLoadingOnStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sector_builder::spawn_all,
                gate_builder::spawn_all,
                station_builder::spawn_all,
                ship_builder::spawn_all,
            )
                .after(initialize_data)
                .after(test_universe::load_test_universe)
                .chain(),
        );
    }
}
