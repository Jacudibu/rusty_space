mod components;
mod gizmos;
mod mouse_cursor;
mod mouse_interaction;
mod mouse_systems;
mod plugin;
mod selection_change_listener;

pub use {components::*, mouse_cursor::MouseCursor, plugin::EntitySelectionPlugin};

/// The maximum delay allowed for a click to count as double-click in milliseconds.
///
/// Windows defaults to 500ms, but that feels awfully long in a game context.
pub const DOUBLE_CLICK_TIME: u128 = 300;

/// The maximum duration for a mouse press to be pressed to count as a click.
///
/// The average mouse click lasts about 85 milliseconds
pub const CLICK_TIME: u128 = 100;
