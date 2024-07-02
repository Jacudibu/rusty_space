use crate::utils::interpolation;
use bevy::input::mouse::MouseWheel;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
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

// Lower values mean faster slowdowns
const MOVEMENT_SLOWDOWN: f32 = 0.075;
const ZOOM_SLOWDOWN: f32 = 0.1;

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
    let (mut projection, zoom_factor) = query.get_single_mut().unwrap();
    if projection.scale == zoom_factor.target {
        return;
    }

    projection.scale = interpolation::weighted_average(
        projection.scale,
        zoom_factor.target,
        ZOOM_SLOWDOWN / time.delta_seconds(),
    );
}

pub fn animate_smooth_camera_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SmoothMoving), With<MainCamera>>,
) {
    let (mut transform, smooth_move) = query.get_single_mut().unwrap();
    if transform.translation == smooth_move.target {
        return;
    }

    transform.translation = Vec3 {
        x: interpolation::weighted_average(
            transform.translation.x,
            smooth_move.target.x,
            MOVEMENT_SLOWDOWN / time.delta_seconds(),
        ),
        y: interpolation::weighted_average(
            transform.translation.y,
            smooth_move.target.y,
            MOVEMENT_SLOWDOWN / time.delta_seconds(),
        ),
        z: transform.translation.z,
    };
}
