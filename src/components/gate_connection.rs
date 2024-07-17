use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct GateConnectionComponent {
    pub render_positions: Vec<Vec3>,
}
