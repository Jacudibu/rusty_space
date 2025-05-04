use bevy::prelude::{Component, Entity};
use common::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;

pub mod test_app;

pub fn mock_entity_id<T: Component>(id: u32) -> TypedEntityWrapper<T> {
    Entity::from_raw(id).into()
}
