use crate::entity_selection::MouseCursor;
use crate::entity_selection::gizmos::{
    MouseInteractionGizmos, draw_colliders, draw_cursor_circle, draw_mouse_interactions,
};
use crate::entity_selection::mouse_cursor::update_mouse_cursor_position;
use crate::entity_selection::mouse_systems::*;
use crate::entity_selection::selection_change_listener::on_selection_changed;
use bevy::prelude::{App, AppGizmoBuilder, IntoScheduleConfigs, Plugin, PreUpdate, Update};

const DRAW_DEBUG_GIZMOS: bool = false;

pub struct EntitySelectionPlugin;
impl Plugin for EntitySelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MouseInteractionGizmos>()
            .insert_resource(MouseCursor::default())
            .add_systems(PreUpdate, update_mouse_cursor_position)
            .add_systems(
                Update,
                (
                    process_mouse_clicks,
                    update_active_mouse_interaction,
                    draw_mouse_interactions,
                    on_selection_changed
                        .after(process_mouse_clicks)
                        .after(update_active_mouse_interaction),
                ),
            );

        if DRAW_DEBUG_GIZMOS {
            app.add_systems(Update, (draw_colliders, draw_cursor_circle));
        }
    }
}
