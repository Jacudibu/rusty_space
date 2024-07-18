use crate::initialize_data;
use crate::persistence::builder::{gate, sector, ship, station};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoSystemConfigs;

pub struct UniverseSaveDataLoadingOnStartupPlugin;

// TODO: Instead of doing this all at once (and chained), separate loading into multiple States and
//       only load X entities per frame, so the app remains responsive and we can have a fancy
//       looking loading bar... plus we can then invoke it multiple times per run, yay!
impl Plugin for UniverseSaveDataLoadingOnStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                sector::spawn_all,
                gate::spawn_all,
                station::spawn_all,
                ship::spawn_all,
            )
                .after(initialize_data)
                .chain(),
        );
    }
}
