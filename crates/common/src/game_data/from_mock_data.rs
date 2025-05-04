use bevy::prelude::World;
use leafwing_manifest::manifest::Manifest;

/// A manifest with this trait can be created from hardcoded mock data.
pub trait FromMockData: Manifest {
    /// Returns hardcoded data, useful for testing.
    #[must_use]
    fn from_mock_data(world: &mut World) -> Self;
}
