use bevy::math::VectorSpace;
use bevy::prelude::{Component, Dir2, Rot2, Transform, Vec2};
use hexx::{Quat, Vec3};

#[derive(Component)]
pub struct SimulationTransform {
    pub translation: Vec2,
    pub rotation: Rot2,
    last_translation: Vec2,
    last_rotation: Rot2,
}

/// Scale isn't really used within our Simulation and only serves as visual eye candy.
/// Thus it's handled separately, allowing us to parallelize systems which require both
/// `all_transforms: Query<&SimulationTransform>`
/// and
/// `entities: Query<(Entity, &mut SimulationScale, [...])`.
#[derive(Component)]
pub struct SimulationScale {
    pub scale: f32,
    last_scale: f32,
}

impl Default for SimulationScale {
    fn default() -> Self {
        Self {
            scale: 1.0,
            last_scale: 1.0,
        }
    }
}

impl From<f32> for SimulationScale {
    fn from(value: f32) -> Self {
        Self {
            scale: value,
            last_scale: value,
        }
    }
}

impl SimulationTransform {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            rotation: Rot2::IDENTITY,
            last_translation: translation,
            last_rotation: Rot2::IDENTITY,
        }
    }

    pub fn new(translation: Vec2, rotation: Rot2) -> Self {
        Self {
            translation,
            rotation,
            last_translation: translation,
            last_rotation: rotation,
        }
    }

    #[inline]
    pub(in crate::simulation::transform) fn copy_old_values(&mut self) {
        self.last_translation = self.translation;
        self.last_rotation = self.rotation;
    }

    /// Rotate this transform counterclockwise by the given value in radians.
    #[inline]
    pub fn rotate(&mut self, radians: f32) {
        self.rotation *= Rot2::radians(radians);
    }

    /// Returns the current forward direction, depending on the current rotation.
    #[inline]
    #[must_use]
    pub fn forward(&self) -> Dir2 {
        self.rotation * Dir2::Y
    }

    /// Crate a 3D Transform based on self, with the z position set to the provided z_layer.
    #[inline]
    #[must_use]
    pub fn as_transform(&self, z_layer: f32) -> Transform {
        self.as_scaled_transform(z_layer, 1.0)
    }

    /// Crate a 3D Transform based on self, with the z position set to the provided z_layer.
    #[inline]
    #[must_use]
    pub fn as_scaled_transform(&self, z_layer: f32, scale: f32) -> Transform {
        Transform {
            translation: self.translation.extend(z_layer),
            rotation: Quat::from_rotation_z(self.rotation.as_radians()),
            scale: Vec3::splat(scale),
        }
    }

    /// Updates the transform if there were any changes detected during [Self::copy_old_values].
    #[inline]
    pub(in crate::simulation::transform) fn update_transform(
        &self,
        transform: &mut Transform,
        overstep_fraction: f32,
    ) {
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
    }
}

impl SimulationScale {
    #[inline]
    pub fn update_transform(&self, transform: &mut Transform, overstep_fraction: f32) {
        let scale = self.last_scale.lerp(self.scale, overstep_fraction);
        transform.scale.x = scale;
        transform.scale.y = scale;
        transform.scale.z = scale;
    }

    #[inline]
    pub(in crate::simulation::transform) fn copy_old_values(&mut self) {
        self.last_scale = self.scale;
    }
}
