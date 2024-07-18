use bevy::prelude::{Rot2, Vec2};

pub struct SimulationTransform {
    pub translation: Vec2,
    pub rotation: Rot2,
}

impl SimulationTransform {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            rotation: Rot2::IDENTITY,
        }
    }
}
