// use crate::simulation::prelude::SimulationTime;
// use bevy::app::{App, Plugin};
// use bevy::ecs::system::SystemParam;
// use bevy::ecs::system::lifetimeless::SRes;
// use bevy::prelude::{Commands, Component, Startup};
// use iyes_perf_ui::entry::PerfUiEntry;
// use iyes_perf_ui::prelude::*;
//
// pub struct DiagnosticsPlugin;
// impl Plugin for DiagnosticsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
//             .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
//             .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
//             .add_plugins(PerfUiPlugin)
//             .add_perf_ui_simple_entry::<PerfUiTickDisplay>()
//             .add_systems(Startup, init);
//     }
// }
//
// fn init(mut commands: Commands) {
//     commands.spawn((
//         PerfUiRoot {
//             values_col_width: 32.0,
//             ..Default::default()
//         },
//         PerfUiTickDisplay::default(),
//         PerfUiEntryFPS {
//             #[cfg(debug_assertions)]
//             label: "FPS [RUNNING IN DEBUG]".into(),
//             ..Default::default()
//         },
//         PerfUiEntryFPSWorst::default(),
//         PerfUiEntryFrameTime::default(),
//         PerfUiEntryFrameTimeWorst::default(),
//         PerfUiEntryEntityCount {
//             digits: 7,
//             threshold_highlight: None,
//             color_gradient: ColorGradient::new_preset_gyr(50000.0, 200000.0, 500000.0).unwrap(),
//             ..Default::default()
//         },
//         PerfUiEntryCpuUsage {
//             label: "System CPU Usage".into(),
//             ..Default::default()
//         },
//         PerfUiEntryMemUsage {
//             label: "System RAM Usage".into(),
//             ..Default::default()
//         },
//     ));
// }
//
// /// Custom Perf UI entry to show the current game tick
// #[derive(Component)]
// #[require(PerfUiRoot)]
// struct PerfUiTickDisplay {
//     /// Required to ensure the entry appears in the correct place in the Perf UI
//     pub sort_key: i32,
// }
//
// impl Default for PerfUiTickDisplay {
//     fn default() -> Self {
//         Self {
//             sort_key: iyes_perf_ui::utils::next_sort_key(),
//         }
//     }
// }
//
// impl PerfUiEntry for PerfUiTickDisplay {
//     type SystemParam = SRes<SimulationTime>;
//     type Value = u32;
//
//     fn label(&self) -> &str {
//         "Tick"
//     }
//
//     fn sort_key(&self) -> i32 {
//         self.sort_key
//     }
//
//     fn update_value(
//         &self,
//         param: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
//     ) -> Option<Self::Value> {
//         Some(param.tick())
//     }
// }
