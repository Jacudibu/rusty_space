use bevy::prelude::Component;

#[derive(Component)]
pub struct AsteroidMiningComponent {
    pub amount_per_second: u32,
}
