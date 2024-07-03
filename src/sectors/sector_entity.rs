use bevy::prelude::Entity;

/// Wrapper around [Entity] to guarantee type safety
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SectorEntity(Entity);

impl SectorEntity {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl From<Entity> for SectorEntity {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}
