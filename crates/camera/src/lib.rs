use bevy::prelude::{App, Camera2d, IntoScheduleConfigs, Name, Plugin, Update, in_state};

mod camera_settings;
mod main_camera;
mod panning;
mod zooming;

pub use camera_settings::CameraSettings;

use crate::main_camera::MainCameraComponent;
use common::states::MouseCursorOverUiState;

/// Inserts the main camera and offers ways to control it.
/// May be configured by editing [CameraSettings].
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>();
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
            ),
        );

        // TODO: This should happen during State transitions
        app.world_mut()
            .spawn((Name::new("Main Camera"), Camera2d, MainCameraComponent));
    }
}
