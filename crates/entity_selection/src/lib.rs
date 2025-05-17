pub mod components;
mod gizmos;
pub mod mouse_cursor;
mod mouse_interaction;
mod mouse_systems;
pub mod plugin;
mod selection_change_listener;

/// The maximum delay allowed for a click to count as double-click in milliseconds.
///
/// Windows defaults to 500ms, but that feels awfully long in a game context.
pub(crate) const DOUBLE_CLICK_TIME: u128 = 300;

/// The maximum duration for a mouse press to be pressed to count as a click.
///
/// The average mouse click lasts about 85 milliseconds
pub(crate) const CLICK_TIME: u128 = 100;
