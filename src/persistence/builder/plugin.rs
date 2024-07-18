use crate::initialize_data;
use crate::persistence::builder::{gate, sector, ship, station};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoSystemConfigs;

pub struct UniverseSaveDataLoadingOnStartupPlugin;

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
