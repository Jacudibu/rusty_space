use bevy::prelude::{Component, Entity};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Wrapper around [Entity] to guarantee type safety when storing Entities for specific component combinations.
/// You'll usually want to use the typed aliases of this instead of using this directly.
pub struct TypedEntityWrapper<T: Component>(Entity, PhantomData<T>);

impl<T: Component> Display for TypedEntityWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", std::any::type_name::<T>(), self.0)
    }
}
impl<T: Component> Debug for TypedEntityWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T: Component> From<Entity> for TypedEntityWrapper<T> {
    fn from(value: Entity) -> Self {
        Self(value, PhantomData)
    }
}
impl<T: Component> From<&Entity> for TypedEntityWrapper<T> {
    fn from(value: &Entity) -> Self {
        Self(*value, PhantomData)
    }
}

impl<T: Component> From<TypedEntityWrapper<T>> for Entity {
    fn from(value: TypedEntityWrapper<T>) -> Self {
        value.0
    }
}
impl<T: Component> From<&TypedEntityWrapper<T>> for Entity {
    fn from(value: &TypedEntityWrapper<T>) -> Self {
        value.0
    }
}

impl<T: Component> Copy for TypedEntityWrapper<T> {}
impl<T: Component> Clone for TypedEntityWrapper<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Component> Eq for TypedEntityWrapper<T> {}
impl<T: Component> PartialEq<Self> for TypedEntityWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Component> PartialEq<Entity> for TypedEntityWrapper<T> {
    fn eq(&self, other: &Entity) -> bool {
        &self.0 == other
    }
}

impl<T: Component> PartialEq<TypedEntityWrapper<T>> for Entity {
    fn eq(&self, other: &TypedEntityWrapper<T>) -> bool {
        self == &other.0
    }
}

impl<T: Component> Ord for TypedEntityWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Component> PartialOrd<Self> for TypedEntityWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Component> Hash for TypedEntityWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
