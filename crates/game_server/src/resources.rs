use std::collections::{HashMap, HashSet};

use bevy_ecs::{entity::Entity, prelude::Resource};

use game_shared::messages::BasicResponse;
use naia_bevy_server::{ResponseReceiveKey, RoomKey, UserKey};

#[derive(Resource)]
pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_square_map: HashMap<UserKey, Entity>,
    pub square_to_user_map: HashMap<Entity, UserKey>,
    pub response_keys: HashSet<ResponseReceiveKey<BasicResponse>>,
    pub request_index: u8,
}