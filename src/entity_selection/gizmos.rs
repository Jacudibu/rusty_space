use crate::entity_selection::mouse_interaction::MouseInteraction;
use bevy::math::Rot2;
use bevy::prelude::{GizmoConfigGroup, Gizmos, Reflect, Res};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MouseInteractionGizmos;

pub const RADIUS: f32 = 8.0;

pub fn draw_mouse_interactions(
    mut gizmos: Gizmos<MouseInteractionGizmos>,
    mouse_interaction: Option<Res<MouseInteraction>>,
) {
    let Some(mouse_interaction) = mouse_interaction else {
        return;
    };

    if !mouse_interaction.counts_as_drag() {
        return;
    }

    let size = mouse_interaction.origin - mouse_interaction.current;
    let origin = mouse_interaction.origin - size * 0.5;

    gizmos.rect_2d(
        origin,
        Rot2::default(),
        size,
        bevy::color::palettes::css::YELLOW_GREEN,
    );
}
