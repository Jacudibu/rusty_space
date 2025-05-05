use bevy::math::Vec2;

/// Gate Traversal is split up into different states
/// Ranging from "Getting sucked into it" to "traversing along the connection at full speed"
#[derive(Default)]
pub enum GateTraversalState {
    /// The task has just been created, used to set up starting values
    #[default]
    JustCreated,

    /// The ship is still speeding up.
    BlendingIntoMotion { origin: Vec2 },

    /// The ship is zooming along the line at full speed.
    TraversingLine,
}
