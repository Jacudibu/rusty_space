use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Default)]
pub struct MouseCursor {
    pub screen_space: Option<Vec2>,
    pub world_space: Option<Vec2>,
}
