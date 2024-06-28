use crate::entity_selection::gizmos::{draw_mouse_interactions, MouseInteractionGizmos};
use crate::entity_selection::mouse_cursor::update_mouse_cursor_position;
use crate::entity_selection::mouse_handler::*;
use crate::entity_selection::selection_change_listener::on_selection_changed;
use crate::entity_selection::MouseCursor;
use bevy::prelude::{App, AppGizmoBuilder, IntoSystemConfigs, Plugin, PreUpdate, Update};

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
    }
}
