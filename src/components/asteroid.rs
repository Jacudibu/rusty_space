use bevy::prelude::{Component, FloatExt, Vec3};

#[derive(Component)]
pub struct Asteroid {
    pub ore: u32,
}

impl Asteroid {
    pub fn scale_depending_on_current_ore_volume(&self) -> Vec3 {
        const MIN: f32 = 0.3;
        const MAX: f32 = 1.0;
        let t = self.ore as f32 / 100.0;

        Vec3::splat(MIN.lerp(MAX, t))
    }
}
