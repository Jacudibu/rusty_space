use crate::utils::SimulationTime;
use bevy::app::{App, FixedPreUpdate};
use bevy::math::VectorSpace;
use bevy::prelude::{
    Component, DetectChanges, Dir2, Fixed, Local, Mut, Plugin, Query, Res, Rot2, Time, Transform,
    Update, Vec2, ViewVisibility,
};
use hexx::{Quat, Vec3};

#[derive(Component)]
pub struct SimulationTransform {
    pub translation: Vec2,
    pub rotation: Rot2,
    pub scale: f32,
    last_translation: Vec2,
    last_rotation: Rot2,
    last_scale: f32,
    did_change: bool,
}

/// Interpolates the transforms used for the visual representation to their respective simulation values.
pub struct SimulationTransformPlugin;
impl Plugin for SimulationTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, copy_old_transform_values);
        app.add_systems(Update, interpolate_transforms);
    }
}

fn copy_old_transform_values(mut transforms: Query<Mut<SimulationTransform>>) {
    transforms.par_iter_mut().for_each(|mut x| {
        let did_change = x.is_changed();
        x.copy_old_values(did_change);
    });
}

fn interpolate_transforms(
    time: Res<Time<Fixed>>,
    simulation_time: Res<SimulationTime>,
    mut all_ships: Query<(&SimulationTransform, &mut Transform, &ViewVisibility)>,
    mut update_all_next_frame: Local<bool>,
) {
    let update_all = *update_all_next_frame && !simulation_time.is_changed();
    if simulation_time.is_changed() {
        *update_all_next_frame = true;
    }

    let overstep_fraction = time.overstep_fraction();

    if update_all {
        all_ships
            .par_iter_mut()
            .for_each(|(simulation_transform, mut transform, _)| {
                if !simulation_transform.did_change {
                    return;
                }

                simulation_transform.update_transform(&mut transform, overstep_fraction);
            });
    } else {
        all_ships
            .par_iter_mut()
            .for_each(|(simulation_transform, mut transform, visibility)| {
                if !visibility.get() || !simulation_transform.did_change {
                    return;
                }

                simulation_transform.update_transform(&mut transform, overstep_fraction);
            });
    }
}

impl SimulationTransform {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            rotation: Rot2::IDENTITY,
            last_translation: translation,
            last_rotation: Rot2::IDENTITY,
            scale: 1.0,
            last_scale: 1.0,
            did_change: false,
        }
    }

    pub fn new(translation: Vec2, rotation: Rot2, scale: f32) -> Self {
        Self {
            translation,
            rotation,
            scale,
            last_translation: translation,
            last_rotation: rotation,
            last_scale: scale,
            did_change: false,
        }
    }

    #[inline]
    fn copy_old_values(&mut self, did_change: bool) {
        if did_change {
            self.last_translation = self.translation;
            self.last_rotation = self.rotation;
            self.last_scale = self.scale;
        }
        self.did_change = did_change
    }

    /// Rotate this transform counterclockwise by the given value in radians.
    #[inline]
    pub fn rotate(&mut self, radians: f32) {
        self.rotation *= Rot2::radians(radians);
    }

    /// Returns the current forward direction, depending on the current rotation.
    #[inline]
    pub fn forward(&self) -> Dir2 {
        self.rotation * Dir2::Y
    }

    #[inline]
    pub fn set_translation_and_skip_interpolation(&mut self, translation: Vec2) {
        self.translation = translation;
        self.last_translation = translation;
    }

    /// Crate a 3D Transform based on self, with the z position set to the provided z_layer.
    #[inline]
    pub fn as_transform(&self, z_layer: f32) -> Transform {
        Transform {
            translation: self.translation.extend(z_layer),
            rotation: Quat::from_rotation_z(self.rotation.as_radians()),
            scale: Vec3::splat(self.scale),
        }
    }

    #[inline]
    fn update_transform(&self, transform: &mut Transform, overstep_fraction: f32) {
        let interpolated_position = self
            .last_translation
            .lerp(self.translation, overstep_fraction);
        transform.translation.x = interpolated_position.x;
        transform.translation.y = interpolated_position.y;

        let rotation = self.last_rotation.nlerp(self.rotation, overstep_fraction);

        let theta = rotation.as_radians();
        let half_theta = theta * 0.5;
        transform.rotation.w = half_theta.cos();
        transform.rotation.z = half_theta.sin();

        let scale = self.last_scale.lerp(self.scale, overstep_fraction);
        transform.scale.x = scale;
        transform.scale.y = scale;
        transform.scale.z = scale;
    }
}
