use naia_shared::{Channel, ChannelDirection, ChannelMode, Protocol, ProtocolPlugin, TickBufferSettings};

#[derive(Channel)]
pub struct PlayerCommandChannel;

pub struct ChannelsPlugin;

impl ProtocolPlugin for ChannelsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol.add_channel::<PlayerCommandChannel>(
            ChannelDirection::ClientToServer,
            ChannelMode::TickBuffered(TickBufferSettings::default())
        );
    }
}