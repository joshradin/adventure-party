use crate::Entity;
use naia_shared::{
    BigMap, ComponentFieldUpdate, ComponentKind, ComponentUpdate, GlobalWorldManagerType,
    LocalEntityAndGlobalEntityConverter, ReplicaDynMutWrapper, ReplicaDynRefWrapper,
    ReplicaMutWrapper, ReplicaRefWrapper, Replicate, SerdeErr, WorldMutType,
    WorldRefType,
};
use std::collections::HashMap;

/// A default world, which implements [WorldRefType] and [WorldMutType] so that Naia can use it to
/// store Entities/Components for macroquad
pub struct World {
    pub entities: BigMap<Entity, HashMap<ComponentKind, Box<dyn Replicate>>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            entities: BigMap::new(),
        }
    }
}

impl World {
    pub fn proxy(&self) -> WorldRef {
        WorldRef { world: self }
    }

    pub fn proxy_mut(&mut self) -> WorldMut {
        WorldMut { world: self }
    }
}

pub struct WorldRef<'w> {
    world: &'w World,
}

pub struct WorldMut<'w> {
    world: &'w mut World,
}

impl<'w> WorldRefType<Entity> for WorldMut<'w> {
    fn has_entity(&self, entity: &Entity) -> bool {
        todo!()
    }

    fn entities(&self) -> Vec<Entity> {
        todo!()
    }

    fn has_component<R: Replicate>(&self, entity: &Entity) -> bool {
        todo!()
    }

    fn has_component_of_kind(&self, entity: &Entity, component_kind: &ComponentKind) -> bool {
        todo!()
    }

    fn component<'a, R: Replicate>(&'a self, entity: &Entity) -> Option<ReplicaRefWrapper<'a, R>> {
        todo!()
    }

    fn component_of_kind<'a>(&'a self, entity: &Entity, component_kind: &ComponentKind) -> Option<ReplicaDynRefWrapper<'a>> {
        todo!()
    }
}
