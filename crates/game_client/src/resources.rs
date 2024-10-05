use bevy::prelude::{ColorMaterial, Entity, Handle, Mesh, Resource};
use game_shared::messages::{BasicResponse, KeyCommand};
use naia_bevy_client::{CommandHistory, ResponseReceiveKey};
use std::collections::HashSet;

#[derive(Resource)]
pub struct Global {
    pub owned_entity: Option<OwnedEntity>,
    pub cursor_entity: Option<Entity>,
    pub queued_command: Option<KeyCommand>,
    pub command_history: CommandHistory<KeyCommand>,
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub purple: Handle<ColorMaterial>,
    pub orange: Handle<ColorMaterial>,
    pub aqua: Handle<ColorMaterial>,
    pub circle: Handle<Mesh>,
    pub response_keys: HashSet<ResponseReceiveKey<BasicResponse>>,
    pub request_index: u8,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            owned_entity: None,
            cursor_entity: None,
            queued_command: None,
            command_history: Default::default(),
            red: Default::default(),
            blue: Default::default(),
            yellow: Default::default(),
            green: Default::default(),
            white: Default::default(),
            purple: Default::default(),
            orange: Default::default(),
            aqua: Default::default(),
            circle: Default::default(),
            response_keys: Default::default(),
            request_index: 0,
        }
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
