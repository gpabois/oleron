use std::ops::{Deref, DerefMut};

use crate::r#box::Box;

pub trait BoxSystem<'a, U: Default> {
    type EntityId;
    type BoxRef: Deref<Target = Box<U>>;
    type BoxMutRef: Deref<Target = Box<U>> + DerefMut;

    fn borrow_box(&'a self, entity: &Self::EntityId) -> Option<Self::BoxRef>;
    fn borrow_mut_box(&'a self, entity: &Self::EntityId) -> Option<Self::BoxMutRef>;
    fn bind_box(&mut self, entity: &Self::EntityId, bx: Box<U>);
    fn bind_default_box(&mut self, entity: &Self::EntityId) {
        self.bind_box(entity, Box::default());
    }
}