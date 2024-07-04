use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Startup};
use iyes_perf_ui::prelude::*;

pub struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryFrameTimeWorst::default(),
        PerfUiEntryEntityCount {
            digits: 7,
            threshold_highlight: None,
            color_gradient: ColorGradient::new_preset_gyr(50000.0, 200000.0, 500000.0).unwrap(),
            ..Default::default()
        },
        PerfUiEntryCpuUsage::default(),
        PerfUiEntryMemUsage::default(),
    ));
}
