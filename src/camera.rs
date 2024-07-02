use bevy::input::mouse::MouseWheel;
use bevy::input::ButtonInput;
use bevy::math::{Vec3, VectorSpace};
use bevy::prelude::{
    Component, EventReader, KeyCode, OrthographicProjection, Query, Res, Time, Transform, With,
};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct SmoothZooming {
    target: f32,
}
impl Default for SmoothZooming {
    fn default() -> Self {
        Self { target: 1.0 }
    }
}

#[derive(Component, Default)]
pub struct SmoothMoving {
    target: Vec3,
}

const CAMERA_SPEED: f32 = 1000.0;
const ZOOM_SPEED_KEYBOARD: f32 = 3.0;
const ZOOM_SPEED_MOUSE: f32 = 0.2;
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 4.0;

const MOVEMENT_SLOWDOWN: f32 = 13.0;
const ZOOM_SLOWDOWN: f32 = 10.0;

pub fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut SmoothMoving, &OrthographicProjection), With<MainCamera>>,
) {
    let mut dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir.length() < 0.01 {
        return;
    }

    let (mut smooth_moving, projection) = query.get_single_mut().unwrap();
    let zoom_factor = 1.0 / projection.scale;
    smooth_moving.target += ((dir * CAMERA_SPEED) / zoom_factor) * time.delta_seconds();
}

pub fn zoom_camera_with_buttons(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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

    zoom_factor.target += dir * time.delta_seconds() * ZOOM_SPEED_KEYBOARD;
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
    time: Res<Time>,
    mut query: Query<(&mut OrthographicProjection, &SmoothZooming), With<MainCamera>>,
) {
    let (mut projection, smooth_zoom) = query.get_single_mut().unwrap();
    if projection.scale == smooth_zoom.target {
        return;
    }

    let t = time.delta_seconds() * ZOOM_SLOWDOWN;
    projection.scale = if t < 1.0 {
        projection.scale.lerp(smooth_zoom.target, t)
    } else {
        smooth_zoom.target
    };
}

pub fn animate_smooth_camera_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SmoothMoving), With<MainCamera>>,
) {
    let (mut transform, smooth_move) = query.get_single_mut().unwrap();
    if transform.translation == smooth_move.target {
        return;
    }

    let t = time.delta_seconds() * MOVEMENT_SLOWDOWN;
    transform.translation = if t < 1.0 {
        transform.translation.lerp(smooth_move.target, t)
    } else {
        smooth_move.target
    };
}
