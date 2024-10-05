use crate::systems::{events, init, input, sync};
use bevy::prelude::*;
use game_shared::protocol;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};

pub struct Main;

#[derive(SystemSet, Debug, Hash, Clone, Eq, PartialEq)]
struct MainLoop;
#[derive(SystemSet, Debug, Hash, Clone, Eq, PartialEq)]
struct Tick;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some("#mygame-canvas".into()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ClientPlugin::<Main>::new(
            ClientConfig::default(),
            protocol(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, init)
        // Receive client events
        .add_systems(
            Update,
            (
                events::connect_events,
                events::disconnect_events,
                events::reject_events,
                events::spawn_entity_events,
                events::despawn_entity_events,
                events::publish_entity_events,
                events::unpublish_entity_events,
                events::insert_component_events,
                events::update_component_events,
                events::remove_component_events,
                events::message_events,
                events::reject_events,
                events::response_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Tick event
        .configure_sets(Update, Tick.after(ReceiveEvents))
        .add_systems(Update, events::tick_events.in_set(Tick))
        // realtime game loop
        .configure_sets(Update, MainLoop.after(Tick))
        .add_systems(
            Update,
            (
                input::key_input,
                input::cursor_input,
                sync::sync_clientside_sprites,
                sync::sync_cursor_sprite,
                sync::sync_serverside_sprites,
            )
                .chain()
                .in_set(MainLoop),
        )
        .run();
}
