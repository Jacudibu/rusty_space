use crate::camera_settings::CameraSettings;
use crate::main_camera::MainCamera;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{Component, KeyCode, Projection, Query, Real, Res, Time, Transform, With};
use common::constants::BevyResult;

/// How quickly the camera movement should come to a halt.
const MOVEMENT_SLOWDOWN: f32 = 13.0;

pub fn pan_camera(
    settings: Res<CameraSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
    mut query: Query<(&mut SmoothPanning, &Projection), With<MainCamera>>,
) -> BevyResult {
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
        return Ok(());
    }

    let (mut smooth_moving, projection) = query.single_mut()?;
    let Projection::Orthographic(projection) = projection else {
        return Err("Cannot move cameras without Orthographic projections!".into());
    };
    let zoom_factor = 1.0 / projection.scale;
    smooth_moving.target += ((dir * settings.pan_speed) / zoom_factor) * time.delta_secs();

    Ok(())
}

pub fn animate_smooth_camera_panning(
    time: Res<Time<Real>>,
    mut query: Query<(&mut Transform, &SmoothPanning), With<MainCamera>>,
) -> BevyResult {
    let (mut transform, smooth_move) = query.single_mut()?;
    if smooth_move.is_not_needed_this_frame(transform.translation) {
        transform.translation = smooth_move.target;
        return Ok(());
    }

    let t = time.delta_secs() * MOVEMENT_SLOWDOWN;
    transform.translation = if t < 1.0 {
        transform.translation.lerp(smooth_move.target, t)
    } else {
        // Disable smoothing if we got long frame times
        smooth_move.target
    };

    Ok(())
}

#[derive(Component, Default)]
pub struct SmoothPanning {
    pub target: Vec3,
}

impl SmoothPanning {
    /// Returns true if the provided camera translation is close enough to the target location.
    pub fn is_not_needed_this_frame(&self, camera_translation: Vec3) -> bool {
        camera_translation.distance_squared(self.target).abs() < 0.1
    }
}
