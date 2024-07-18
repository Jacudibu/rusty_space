use bevy::app::{App, FixedPreUpdate};
use bevy::math::VectorSpace;
use bevy::prelude::{
    Changed, Component, Fixed, Plugin, Query, Res, Rot2, Time, Transform, Update, Vec2,
};
use hexx::{Quat, Vec3};

#[derive(Component)]
pub struct SimulationTransform {
    pub translation: Vec2,
    pub rotation: Rot2,
    pub scale: f32,
    pub last_translation: Vec2,
    pub last_rotation: Rot2,
    pub last_scale: f32,
}

/// Interpolates the transforms used for the visual representation to their respective simulation values.
pub struct SimulationTransformPlugin;
impl Plugin for SimulationTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, copy_old_transform_values);
        app.add_systems(Update, interpolate_transforms);
    }
}

fn copy_old_transform_values(mut transforms: Query<&mut SimulationTransform>) {
    transforms
        .par_iter_mut()
        .for_each(|mut x| x.copy_old_values());
}

// TODO: is there any way to use Changed<SimulationTransform> specifically for fixed update changes?
fn interpolate_transforms(
    time: Res<Time<Fixed>>,
    mut all_ships: Query<(&SimulationTransform, &mut Transform)>,
) {
    let overstep_fraction = time.overstep_fraction();
    all_ships
        .par_iter_mut()
        .for_each(|(simulation_transform, mut transform)| {
            let interpolated_position = simulation_transform
                .last_translation
                .lerp(simulation_transform.translation, overstep_fraction);
            transform.translation.x = interpolated_position.x;
            transform.translation.y = interpolated_position.y;

            let rotation = simulation_transform
                .last_rotation
                .nlerp(simulation_transform.rotation, overstep_fraction);

            let theta = rotation.as_radians();
            let half_theta = theta * 0.5;
            transform.rotation.w = half_theta.cos();
            transform.rotation.z = half_theta.sin();

            let scale = simulation_transform
                .last_scale
                .lerp(simulation_transform.scale, overstep_fraction);
            transform.scale.x = scale;
            transform.scale.y = scale;
            transform.scale.z = scale;
        });
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
        }
    }

    pub fn copy_old_values(&mut self) {
        self.last_translation = self.translation;
        self.last_rotation = self.rotation;
        self.last_scale = self.scale;
    }

    pub fn rotate(&mut self, amount: f32) {
        self.rotation *= Rot2::radians(amount);
    }

    pub fn forward(&self) -> Vec2 {
        self.rotation * Vec2::Y
    }

    pub fn as_transform(&self, z_layer: f32) -> Transform {
        Transform {
            translation: self.translation.extend(z_layer),
            rotation: Quat::from_rotation_z(self.rotation.as_radians()),
            scale: Vec3::splat(self.scale),
        }
    }
}
