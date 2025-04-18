use crate::camera::main_camera::MainCamera;
use crate::camera::{ZOOM_SPEED_KEYBOARD, ZOOM_SPEED_MOUSE};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::math::VectorSpace;
use bevy::prelude::{
    Component, EventReader, KeyCode, OrthographicProjection, Query, Real, Res, Time, With,
};

/// How far can we zoom in?
const MIN_ZOOM: f32 = 0.25;
/// How far can we zoom out?
const MAX_ZOOM: f32 = 6.0;
/// How swiftly should the zooming motion slow down?
const ZOOM_SLOWDOWN: f32 = 10.0;

pub fn zoom_camera_with_buttons(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
    mut query: Query<&mut SmoothZooming, With<MainCamera>>,
) {
    let mut dir: f32 = 0.0;
    if keys.pressed(KeyCode::KeyR) {
        dir += 1.0;
    }
    if keys.pressed(KeyCode::KeyF) {
        dir -= 1.0;
    }

    if dir.abs() < 0.01 {
        return;
    }

    let mut zoom_factor = query.get_single_mut().unwrap();

    zoom_factor.target += dir * time.delta_secs() * ZOOM_SPEED_KEYBOARD;
    zoom_factor.target = zoom_factor.target.clamp(MIN_ZOOM, MAX_ZOOM);
}

pub fn zoom_camera_with_scroll_wheel(
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<&mut SmoothZooming, With<MainCamera>>,
) {
    for event in scroll_event.read() {
        let mut zoom_factor = query.get_single_mut().unwrap();
        zoom_factor.target += -event.y * ZOOM_SPEED_MOUSE;
        zoom_factor.target = zoom_factor.target.clamp(MIN_ZOOM, MAX_ZOOM);
    }
}

pub fn animate_smooth_camera_zoom(
    time: Res<Time<Real>>,
    mut query: Query<(&mut OrthographicProjection, &SmoothZooming), With<MainCamera>>,
) {
    let (mut projection, smooth_zoom) = query.get_single_mut().unwrap();
    if projection.scale == smooth_zoom.target {
        return;
    }

    let t = time.delta_secs() * ZOOM_SLOWDOWN;
    projection.scale = if t < 1.0 {
        projection.scale.lerp(smooth_zoom.target, t)
    } else {
        smooth_zoom.target
    };
}

#[derive(Component)]
pub struct SmoothZooming {
    target: f32,
}

impl Default for SmoothZooming {
    fn default() -> Self {
        Self { target: 1.0 }
    }
}
