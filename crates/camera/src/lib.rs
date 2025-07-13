use bevy::prelude::{
    App, Camera2d, Commands, Entity, IntoScheduleConfigs, Name, OnEnter, OnExit, Plugin, Query,
    Update, With, in_state,
};

mod camera_settings;
mod main_camera;
mod panning;
mod zooming;

pub use camera_settings::CameraSettings;

pub use crate::main_camera::MainCamera;
use common::states::{ApplicationState, MouseCursorOverUiState};

/// Inserts the main camera and offers ways to control it.
/// May be configured by editing [CameraSettings].
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>();
        app.add_systems(OnEnter(ApplicationState::InGame), spawn_camera);
        app.add_systems(OnExit(ApplicationState::InGame), despawn_camera);
        app.add_systems(
            Update,
            (
                panning::pan_camera,
                panning::animate_smooth_camera_panning.after(panning::pan_camera),
                zooming::zoom_camera_with_buttons,
                zooming::zoom_camera_with_scroll_wheel
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI)),
                zooming::animate_smooth_camera_zoom
                    .after(zooming::zoom_camera_with_scroll_wheel)
                    .after(zooming::zoom_camera_with_buttons),
            )
                .run_if(in_state(ApplicationState::InGame)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), Camera2d, MainCamera));
}

fn despawn_camera(mut commands: Commands, camera: Query<Entity, With<MainCamera>>) {
    for x in camera {
        commands.entity(x).despawn();
    }
}
