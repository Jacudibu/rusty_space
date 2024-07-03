use bevy::prelude::Entity;

/// Wrapper around [Entity] to guarantee type safety
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GateEntity(Entity);

impl GateEntity {
    pub fn get(&self) -> Entity {
        self.0
    }
}

impl From<Entity> for GateEntity {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}
