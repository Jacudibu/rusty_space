use bevy::prelude::{Camera, GlobalTransform, Query, ResMut, Resource, Vec2, Window};

#[derive(Resource, Default)]
pub struct MouseCursor {
    pub screen_space: Option<Vec2>,
    pub world_space: Option<Vec2>,
}

pub fn update_mouse_cursor_position(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<MouseCursor>,
) {
    if let Some(position) = windows.single().cursor_position() {
        let (camera, transform) = camera.single();
        let world_pos = camera.viewport_to_world_2d(transform, position);

        cursor.screen_space = Some(position);
        cursor.world_space = world_pos;
    } else {
        cursor.screen_space = None;
        cursor.world_space = None;
    }
}
