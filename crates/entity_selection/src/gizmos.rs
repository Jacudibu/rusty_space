use crate::mouse_interaction::MouseInteraction;
use bevy::math::Rot2;
use bevy::prelude::{GizmoConfigGroup, Gizmos, GlobalTransform, Isometry2d, Query, Reflect, Res};
use common::components::{RADIUS_CURSOR, SelectableEntity};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub(crate) struct MouseInteractionGizmos;

pub(crate) fn draw_mouse_interactions(
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
        Isometry2d::new(origin, Rot2::default()),
        size,
        bevy::color::palettes::css::YELLOW_GREEN,
    );
}

pub(crate) fn draw_colliders(
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

pub(crate) fn draw_cursor_circle(
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
