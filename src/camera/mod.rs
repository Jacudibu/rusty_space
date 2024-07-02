mod main_camera;
mod moving;
mod plugin;
mod zooming;

// These should be configurable later on
// (The forever-hardcoded camera constants are in the respective submodules)
const CAMERA_SPEED: f32 = 1000.0;
const ZOOM_SPEED_KEYBOARD: f32 = 3.0;
const ZOOM_SPEED_MOUSE: f32 = 0.2;

pub use main_camera::MainCameraBundle;
pub use plugin::CameraControllerPlugin;
