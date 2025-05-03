use crate::camera::{moving, zooming};
use crate::gui::MouseCursorOverUiState;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update, in_state};

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                moving::move_camera,
                moving::animate_smooth_camera_movement.after(moving::move_camera),
                zooming::zoom_camera_with_buttons,
                zooming::zoom_camera_with_scroll_wheel
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI)),
                zooming::animate_smooth_camera_zoom
                    .after(zooming::zoom_camera_with_scroll_wheel)
                    .after(zooming::zoom_camera_with_buttons),
            ),
        );
    }
}
