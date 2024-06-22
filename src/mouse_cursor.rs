use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::log::info;
use bevy::prelude::{
    Camera, Commands, Entity, EventReader, GlobalTransform, MouseButton, Query, Res, ResMut,
    Resource, Sprite, Transform, Vec2, With,
};
use bevy::window::Window;

#[derive(Resource, Default)]
pub struct MouseCursor {
    pub screen_space: Option<Vec2>,
    pub world_space: Option<Vec2>,
}

#[derive(Resource)]
pub struct SelectedEntities {
    pub entities: Vec<Entity>,
}

pub fn update_cursor_position(
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

pub fn select_entities(
    mut commands: Commands,
    selectables: Query<(Entity, &Transform), With<Sprite>>,
    mouse_cursor: Res<MouseCursor>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
) {
    const RADIUS: f32 = 8.0;

    for event in mouse_button_events.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        // TODO: Multi Selection and stuff
        if event.state != ButtonState::Pressed {
            continue;
        }

        let Some(cursor_world_pos) = mouse_cursor.world_space else {
            continue;
        };

        let entities: Vec<Entity> = selectables
            .iter()
            .filter_map(|(entity, transform)| {
                let x = cursor_world_pos.x - transform.translation.x;
                let y = cursor_world_pos.y - transform.translation.y;

                if x * x + y * y <= RADIUS * RADIUS {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect();

        commands.insert_resource(SelectedEntities { entities })
    }
}
