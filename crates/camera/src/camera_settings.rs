use bevy::prelude::Resource;

/// A resource to store all settings for the camera which might be changed by the user.
#[derive(Resource)]
pub struct CameraSettings {
    pub pan_speed: f32,
    pub zoom_speed_keyboard: f32,
    pub zoom_speed_mouse: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            pan_speed: 1000.0,
            zoom_speed_keyboard: 3.0,
            zoom_speed_mouse: 0.2,
        }
    }
}
