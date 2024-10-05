//! messages

pub use auth::Auth;
use naia_bevy_shared::{Protocol, ProtocolPlugin};

mod auth;

pub struct MessagesPlugin;

impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol.add_message::<Auth>();
    }
}
