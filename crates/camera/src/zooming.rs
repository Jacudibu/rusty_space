use crate::camera_settings::CameraSettings;
use crate::main_camera::MainCamera;
use crate::panning::SmoothPanning;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::math::{Vec3, VectorSpace};
use bevy::prelude::{
    Camera, Component, EventReader, GlobalTransform, KeyCode, Projection, Query, Real, Res, Time,
    Transform, Vec2, Window, With,
};
use common::constants::BevyResult;

/// How far can we zoom in?
const MIN_ZOOM: f32 = 0.25;

/// How far can we zoom out?
/// TODO: Zoom speed should increase with higher values, as the effect slows down heavily
/// Might be easiest to use a local value and just write a little converter function to ease view transitions
const MAX_ZOOM: f32 = 64.0;

/// How swiftly should the zooming motion slow down?
const ZOOM_SLOWDOWN: f32 = 10.0;

pub fn zoom_camera_with_buttons(
    settings: Res<CameraSettings>,
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

    let mut zoom_factor = query.single_mut().unwrap();

    zoom_factor.target += dir * time.delta_secs() * settings.zoom_speed_keyboard;
    zoom_factor.target = zoom_factor.target.clamp(MIN_ZOOM, MAX_ZOOM);
}

pub fn zoom_camera_with_scroll_wheel(
    settings: Res<CameraSettings>,
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<&mut SmoothZooming, With<MainCamera>>,
) {
    for event in scroll_event.read() {
        let mut zoom_factor = query.single_mut().unwrap();
        zoom_factor.target += -event.y * settings.zoom_speed_mouse;
        zoom_factor.target = zoom_factor.target.clamp(MIN_ZOOM, MAX_ZOOM);
    }
}

pub fn animate_smooth_camera_zoom(
    time: Res<Time<Real>>,
    windows: Query<&Window>,
    mut query: Query<
        (
            &Camera,
            &mut Transform,
            &GlobalTransform,
            &mut Projection,
            &mut SmoothPanning,
            &SmoothZooming,
        ),
        With<MainCamera>,
    >,
) -> BevyResult {
    let (
        camera,
        mut camera_transform,
        camera_global_transform,
        mut projection,
        mut smooth_pan,
        smooth_zoom,
    ) = query.single_mut()?;
    let Projection::Orthographic(ref mut projection) = *projection else {
        return Err("Zooming non-orthographic projections is not supported!".into());
    };
    if smooth_zoom.is_not_needed_this_frame(projection.scale) {
        return Ok(());
    }

    let window = windows.single()?;

    // Don't pan to cursor while we the camera is being moved through other means, that feels weird
    let pan_to_mouse = smooth_pan.is_not_needed_this_frame(camera_transform.translation);

    let cursor_world_pos_before = if pan_to_mouse {
        // TODO: Consider caching cursor pos and reuse it in case mouse movement delta is too big
        let logical_cursor_pos = window
            .cursor_position()
            .unwrap_or_else(|| window.size() * 0.5);

        camera.viewport_to_world_2d(camera_global_transform, logical_cursor_pos)?
    } else {
        Vec2::default()
    };

    let old_scale = projection.scale;

    let t = time.delta_secs() * ZOOM_SLOWDOWN;
    projection.scale = if t < 1.0 {
        projection.scale.lerp(smooth_zoom.target, t)
    } else {
        // Disable smoothing if we got long frame times
        smooth_zoom.target
    };

    if (pan_to_mouse) {
        let ratio = projection.scale / old_scale;
        let cam_xy = camera_global_transform.translation().truncate();
        let delta = (1.0 - ratio) * (cursor_world_pos_before - cam_xy);
        camera_transform.translation.x += delta.x;
        camera_transform.translation.y += delta.y;
        smooth_pan.target.x += delta.x;
        smooth_pan.target.y += delta.y;
    }

    Ok(())
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

impl SmoothZooming {
    /// Returns true if the provided camera translation is close enough to the target location.
    pub fn is_not_needed_this_frame(&self, camera_scale: f32) -> bool {
        (camera_scale - self.target).abs() < 0.0005
    }
}
