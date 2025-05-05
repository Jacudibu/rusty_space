use bevy::prelude::SystemSet;

/// System sets for system scheduling.
/// This allows us to avoid circular dependencies between different crates which need to have their systems run in a specific order.
/// (Not sure if this is a smell right now, gotta figure that out over time)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomSystemSets {
    RespawnAsteroids,
}
