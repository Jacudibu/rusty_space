use crate::camera::CAMERA_SPEED;
use crate::camera::main_camera::MainCameraComponent;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{Component, KeyCode, Projection, Query, Real, Res, Time, Transform, With};

const MOVEMENT_SLOWDOWN: f32 = 13.0;

pub fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
    mut query: Query<(&mut SmoothMoving, &Projection), With<MainCameraComponent>>,
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

    let (mut smooth_moving, projection) = query.single_mut().unwrap();
    let Projection::Orthographic(projection) = projection else {
        panic!("We should only ever have orthographic projections");
    };
    let zoom_factor = 1.0 / projection.scale;
    smooth_moving.target += ((dir * CAMERA_SPEED) / zoom_factor) * time.delta_secs();
}

pub fn animate_smooth_camera_movement(
    time: Res<Time<Real>>,
    mut query: Query<(&mut Transform, &SmoothMoving), With<MainCameraComponent>>,
) {
    let (mut transform, smooth_move) = query.single_mut().unwrap();
    if transform.translation == smooth_move.target {
        return;
    }

    let t = time.delta_secs() * MOVEMENT_SLOWDOWN;
    transform.translation = if t < 1.0 {
        transform.translation.lerp(smooth_move.target, t)
    } else {
        smooth_move.target
    };
}

#[derive(Component, Default)]
pub struct SmoothMoving {
    target: Vec3,
}
