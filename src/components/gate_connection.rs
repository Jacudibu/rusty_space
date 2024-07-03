use bevy::math::Vec3;
use bevy::prelude::Component;

/// Used for Rendering Lines between Gates
#[derive(Component)]
pub struct GateConnectionComponent {
    pub render_positions: Vec<Vec3>,
}
