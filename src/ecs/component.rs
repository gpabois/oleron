use std::{hash::Hash, ops::Deref};

use pb_arena::{ArenaId, sync::{Arena, ArenaRef, ArenaMutRef}};
use pb_atomic_hash_map::AtomicHashMap;

pub type ComponentRef<'a, Component> = ArenaRef<'a, Component>;
pub type ComponentMutRef<'a, Component> = ArenaMutRef<'a, Component>;

// A component holder
pub struct Components<EntityId: Hash, Component> {
    entities: AtomicHashMap<EntityId, ArenaId>,
    arena: Arena<Component>
}

impl<EntityId: Hash, Component> Default for Components<EntityId, Component> {
    fn default() -> Self {
        Self { entities: AtomicHashMap::new(100), arena: Arena::new(100, 100) }
    }
}

impl<EntityId: Hash, Component> Clone for Components<EntityId, Component> {
    fn clone(&self) -> Self {
        Self { entities: self.entities.clone(), arena: self.arena.clone() }
    }
}

impl<Entity: Copy + Hash, Component: Clone> Components<Entity, Component> {
    pub fn clone_component(&mut self, src: &Entity, to: &Entity) {
        if let Some(cloned_component) = self.borrow(src).map(|component| component.clone()) {
            self.bind(&to, cloned_component);
        }
    }
}

impl<Entity: Hash + Copy, Component> Components<Entity, Component> {
    pub fn new(block_size: usize) -> Self {
        Self {
            entities: AtomicHashMap::new(100), 
            arena: Arena::new(block_size, 100)
        }
    }

    // Bind a component to an entity
    // If a component is already bound to the entity, replace its value.
    pub fn bind(&mut self, entity: &Entity, component: Component) {
        if let Some(component_id) = self.entities.borrow(entity) {
            *self.arena.borrow_mut(component_id.deref()).unwrap() = component;
        } else {
            let component_id = self.arena.alloc(component);
            self.entities.insert(*entity, component_id);
        }
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.borrow(&entity).is_some()
    }

    pub fn borrow(&self, entity: &Entity) -> Option<ArenaRef<'_, Component>> {
        self.entities.borrow(entity).map(|component_id| self.arena.borrow(component_id.deref())).flatten()
    }

    pub fn borrow_mut(&self, entity: &Entity) -> Option<ArenaMutRef<'_, Component>> {
        self.entities.borrow(entity).map(|component_id| self.arena.borrow_mut(component_id.deref())).flatten()
    }
}

impl<Entity: Hash + Copy, Component: Default> Components<Entity, Component> {

    pub fn bind_default(&mut self, entity: &Entity) {
        self.bind(entity, Default::default())
    }
}
