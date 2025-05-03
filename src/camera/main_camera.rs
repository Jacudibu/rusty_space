use crate::camera::moving::SmoothMoving;
use crate::camera::zooming::SmoothZooming;
use bevy::prelude::{Bundle, Component};

#[derive(Component, Default)]
#[component(immutable)]
pub struct MainCameraComponent;

#[derive(Bundle, Default)]
pub struct MainCameraBundle {
    pub main_camera: MainCameraComponent,
    pub smooth_zooming: SmoothZooming,
    pub smooth_moving: SmoothMoving,
}
