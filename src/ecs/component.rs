use std::collections::BTreeMap;

use pb_arena::{Arena, ArenaId, ArenaRef, ArenaRefMut};

pub type ComponentRef<'a, Component> = ArenaRef<'a, Component>;
pub type ComponentMutRef<'a, Component> = ArenaRefMut<'a, Component>;

// A component holder
pub struct Components<EntityId, Component> {
    entities: BTreeMap<EntityId, ArenaId>,
    arena: Arena<Component>
}

impl<Entity: Ord + Copy, Component> Components<Entity, Component> {
    pub fn new(block_size: usize) -> Self {
        Self {
            entities: Default::default(), 
            arena: Arena::new(block_size)
        }
    }


    // Bind a component to an entity
    // If a component is already bound to the entity, replace its value.
    pub fn bind(&mut self, entity: &Entity, component: Component) {
        if let Some(component_id) = self.entities.get(&entity) {
            *self.arena.borrow_mut(*component_id).unwrap() = component;
        } else {
            let component_id = self.arena.alloc(component);
            self.entities.insert(*entity, component_id);
        }
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.borrow(&entity).is_some()
    }

    pub fn borrow(&self, entity: &Entity) -> Option<ArenaRef<'_, Component>> {
        self.entities.get(&entity).map(|component_id| self.arena.borrow(*component_id)).flatten()
    }

    pub fn borrow_mut(&self, entity: &Entity) -> Option<ArenaRefMut<'_, Component>> {
        self.entities.get(entity).map(|component_id| self.arena.borrow_mut(*component_id)).flatten()
    }
}

impl<Entity: Ord + Copy, Component: Default> Components<Entity, Component> {

    pub fn bind_default(&mut self, entity: &Entity) {
        self.bind(entity, Default::default())
    }

}
