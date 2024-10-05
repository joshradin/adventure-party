use crate::systems::{events, init};
use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_ecs::prelude::*;
use bevy_log::{Level, LogPlugin};
use common::tracing::{init_logging, LoggingOptions, Stdout};
use game_shared::protocol;
use log::info;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use std::time::Duration;

mod resources;
mod systems;

fn main() {
    info!("Starting game server");
    let mut server_config = ServerConfig::default();
    server_config.connection.disconnection_timeout_duration = Duration::from_secs(15);

    App::default()
        // plugins
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin::default())
        .add_plugins(FrameCountPlugin::default())
        // Avoid server running at uncapped frames
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_micros(1000 / 128)))
        .add_plugins(LogPlugin {
            filter: "".to_string(),
            level: Level::DEBUG,
            custom_layer: |_| None,
        })
        .add_plugins(ServerPlugin::new(server_config, protocol()))
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                events::auth_events,
                events::connect_events,
                events::disconnect_events,
                events::error_events,
                events::tick_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::publish_entity_events,
                events::unpublish_entity_events,
                events::insert_component_events,
                events::update_component_events,
                events::remove_component_events,
                events::request_events,
                events::response_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        .run();
}
