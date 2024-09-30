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

impl<'w> WorldRefType<Entity> for WorldRef<'w> {
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

impl<'w> WorldMutType<Entity> for WorldMut<'w> {
    fn spawn_entity(&mut self) -> Entity {
        todo!()
    }

    fn local_duplicate_entity(&mut self, entity: &Entity) -> Entity {
        todo!()
    }

    fn local_duplicate_components(&mut self, mutable_entity: &Entity, immutable_entity: &Entity) {
        todo!()
    }

    fn despawn_entity(&mut self, entity: &Entity) {
        todo!()
    }

    fn component_kinds(&mut self, entity: &Entity) -> Vec<ComponentKind> {
        todo!()
    }

    fn component_mut<'a, R: Replicate>(&'a mut self, entity: &Entity) -> Option<ReplicaMutWrapper<'a, R>> {
        todo!()
    }

    fn component_mut_of_kind<'a>(&'a mut self, entity: &Entity, component_kind: &ComponentKind) -> Option<ReplicaDynMutWrapper<'a>> {
        todo!()
    }

    fn component_apply_update(&mut self, converter: &dyn LocalEntityAndGlobalEntityConverter, entity: &Entity, component_kind: &ComponentKind, update: ComponentUpdate) -> Result<(), SerdeErr> {
        todo!()
    }

    fn component_apply_field_update(&mut self, converter: &dyn LocalEntityAndGlobalEntityConverter, entity: &Entity, component_kind: &ComponentKind, update: ComponentFieldUpdate) -> Result<(), SerdeErr> {
        todo!()
    }

    fn mirror_entities(&mut self, mutable_entity: &Entity, immutable_entity: &Entity) {
        todo!()
    }

    fn mirror_components(&mut self, mutable_entity: &Entity, immutable_entity: &Entity, component_kind: &ComponentKind) {
        todo!()
    }

    fn insert_component<R: Replicate>(&mut self, entity: &Entity, component_ref: R) {
        todo!()
    }

    fn insert_boxed_component(&mut self, entity: &Entity, boxed_component: Box<dyn Replicate>) {
        todo!()
    }

    fn remove_component<R: Replicate>(&mut self, entity: &Entity) -> Option<R> {
        todo!()
    }

    fn remove_component_of_kind(&mut self, entity: &Entity, component_kind: &ComponentKind) -> Option<Box<dyn Replicate>> {
        todo!()
    }

    fn entity_publish(&mut self, global_world_manager: &dyn GlobalWorldManagerType<Entity>, entity: &Entity) {
        todo!()
    }

    fn component_publish(&mut self, global_world_manager: &dyn GlobalWorldManagerType<Entity>, entity: &Entity, component_kind: &ComponentKind) {
        todo!()
    }

    fn entity_unpublish(&mut self, entity: &Entity) {
        todo!()
    }

    fn component_unpublish(&mut self, entity: &Entity, component_kind: &ComponentKind) {
        todo!()
    }

    fn entity_enable_delegation(&mut self, global_world_manager: &dyn GlobalWorldManagerType<Entity>, entity: &Entity) {
        todo!()
    }

    fn component_enable_delegation(&mut self, global_world_manager: &dyn GlobalWorldManagerType<Entity>, entity: &Entity, component_kind: &ComponentKind) {
        todo!()
    }

    fn entity_disable_delegation(&mut self, entity: &Entity) {
        todo!()
    }

    fn component_disable_delegation(&mut self, entity: &Entity, component_kind: &ComponentKind) {
        todo!()
    }
}
