use crate::channels::ChannelsPlugin;
use crate::messages::MessagesPlugin;
use naia_bevy_shared::{LinkConditionerConfig, Protocol};
use std::time::Duration;

pub fn protocol() -> Protocol {
    Protocol::builder()
        .tick_interval(Duration::from_millis(1000 / 60))
        .link_condition(LinkConditionerConfig::good_condition())
        .enable_client_authoritative_entities()
        .add_plugin(ChannelsPlugin)
        .add_plugin(MessagesPlugin)
        .build()
}
