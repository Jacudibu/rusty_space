use crate::components::{SelectableEntity, RADIUS_CURSOR};
use crate::entity_selection::mouse_interaction::MouseInteraction;
use bevy::math::Rot2;
use bevy::prelude::{GizmoConfigGroup, Gizmos, GlobalTransform, Query, Reflect, Res};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MouseInteractionGizmos;

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

pub fn draw_colliders(
    mut gizmos: Gizmos<MouseInteractionGizmos>,
    query: Query<(&GlobalTransform, &SelectableEntity)>,
) {
    for (transform, selectable) in query.iter() {
        gizmos.circle_2d(
            transform.translation().truncate(),
            selectable.radius(),
            bevy::color::palettes::css::GREEN,
        );
    }
}

pub fn draw_cursor_circle(
    mut gizmos: Gizmos<MouseInteractionGizmos>,
    cursor: Option<Res<MouseInteraction>>,
) {
    if let Some(cursor) = cursor {
        gizmos.circle_2d(
            cursor.current,
            RADIUS_CURSOR,
            bevy::color::palettes::css::GREEN,
        );
    }
}
