use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct Global {}

impl Default for Global {
    fn default() -> Self {
        Self {}
    }
}

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed: Entity, predicted: Entity) -> Self {
        Self {
            confirmed,
            predicted,
        }
    }
}
