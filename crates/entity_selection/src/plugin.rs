use crate::gizmos::{
    MouseInteractionGizmos, draw_colliders, draw_cursor_circle, draw_mouse_interactions,
};
use crate::mouse_cursor::{MouseCursor, update_mouse_cursor_position};
use crate::mouse_systems::{process_mouse_clicks, update_active_mouse_interaction};
use crate::selection_change_listener::on_selection_changed;
use bevy::prelude::{
    App, AppGizmoBuilder, IntoScheduleConfigs, Plugin, PreUpdate, Update, in_state,
};
use common::states::ApplicationState;

const DRAW_DEBUG_GIZMOS: bool = false;

pub struct EntitySelectionPlugin;
impl Plugin for EntitySelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MouseInteractionGizmos>()
            .insert_resource(MouseCursor::default())
            .add_systems(
                PreUpdate,
                update_mouse_cursor_position.run_if(in_state(ApplicationState::InGame)),
            )
            .add_systems(
                Update,
                (
                    process_mouse_clicks,
                    update_active_mouse_interaction,
                    draw_mouse_interactions,
                    on_selection_changed
                        .after(process_mouse_clicks)
                        .after(update_active_mouse_interaction),
                )
                    .run_if(in_state(ApplicationState::InGame)),
            );

        if DRAW_DEBUG_GIZMOS {
            app.add_systems(Update, (draw_colliders, draw_cursor_circle));
        }
    }
}
