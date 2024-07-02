use crate::camera::moving::SmoothMoving;
use crate::camera::zooming::SmoothZooming;
use bevy::prelude::{Bundle, Component};

#[derive(Component, Default)]
pub struct MainCamera;

#[derive(Bundle, Default)]
pub struct MainCameraBundle {
    pub main_camera: MainCamera,
    pub smooth_zooming: SmoothZooming,
    pub smooth_moving: SmoothMoving,
}
