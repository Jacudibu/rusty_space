use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{
    Component, KeyCode, OrthographicProjection, Query, Res, Time, Transform, With,
};

#[derive(Component)]
pub struct MainCamera;

const CAMERA_SPEED: f32 = 100.0;

pub fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
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

    camera.get_single_mut().unwrap().translation += dir * CAMERA_SPEED * time.delta_seconds();
}

pub fn zoom_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut projection: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut dir: f32 = 0.0;
    if keys.just_pressed(KeyCode::KeyR) {
        dir += 1.0;
    }
    if keys.just_pressed(KeyCode::KeyF) {
        dir -= 1.0;
    }

    if dir.abs() < 0.01 {
        return;
    }

    if dir > 0.0 {
        projection.get_single_mut().unwrap().scale *= 1.5;
    } else {
        projection.get_single_mut().unwrap().scale /= 1.5;
    }
}
